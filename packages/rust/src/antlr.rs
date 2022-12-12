#![allow(non_snake_case)]


use antlr_rust::tree::Visitable;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::rule_context::RuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor};
use antlr_rust::{TidExt, InputStream};

use crate::{RelationTuple, RelationTupleParseError};
use crate::common::relationtuple::{RelationTupleBuilder, Subject};
pub use relationtuplelexer::*;
pub use relationtuplelistener::*;
pub use relationtupleparser::*;
pub use relationtuplevisitor::*;

pub mod relationtuplelexer;
pub mod relationtuplelistener;
pub mod relationtupleparser;
pub mod relationtuplevisitor;

pub struct MyRelationTupleParser {
    pub(crate) relation_tuple_builder: RelationTupleBuilder, //FIXME: change to private with errors vector. Then impl a getter which returns a Result<RelationTuple, Errors>
    //relation_tuple_errors: 
}

impl ParseTreeVisitor<'_, RelationTupleParserContextType> for MyRelationTupleParser {}

impl RelationTupleVisitor<'_> for MyRelationTupleParser {
    fn visit_relationTuple(&mut self, ctx: &RelationTupleContext<'_>) {
        self.visit_children(ctx);

        if let Some(relation) = ctx.relation.as_ref() {
            self.relation_tuple_builder
                .relation(relation.get_text().into());
        }
    }

    fn visit_namespacedObject(&mut self, ctx: &NamespacedObjectContext<'_>) {
        self.visit_children(ctx);

        #[allow(non_snake_case)]
        if ctx.get_parent_ctx().unwrap().is::<SubjectSetContext>() {
            return;
        }

        if let Some(namespace) = ctx.namespace.as_ref() {
            self.relation_tuple_builder
                .namespace(namespace.get_text().into());
        }

        if let Some(object) = ctx.object.as_ref() {
            self.relation_tuple_builder.object(object.get_text().into());
        }
    }

    fn visit_subject(&mut self, ctx: &SubjectContext<'_>) {
        self.visit_children(ctx);
    }

    fn visit_subjectId(&mut self, ctx: &SubjectIdContext<'_>) {
        self.visit_children(ctx);

        self.relation_tuple_builder
            .subject(Subject::Id(ctx.get_text()));
    }
    fn visit_subjectSet(&mut self, ctx: &SubjectSetContext<'_>) {
        self.visit_children(ctx);

        if let Some(namespace_object) = ctx.subjectNamespacedObject.as_ref() {
            if let (Some(namespace), Some(object)) = (
                namespace_object.namespace.as_ref(),
                namespace_object.object.as_ref(),
            ) {
                self.relation_tuple_builder.subject(Subject::Set {
                    namespace: namespace.get_text().into(),
                    object: object.get_text().into(),
                    relation: ctx.subjectRelation.as_ref().map(|x| x.get_text().into()),
                });
            }
        }
    }
}


impl RelationTuple {
    pub fn from_str(relation_tuple: &str) -> Result<RelationTuple, RelationTupleParseError> {
        let relation_tuple = relation_tuple.trim();
        if relation_tuple.is_empty() {
            RelationTupleBuilder::default().build()?;
        }

        // FIXME: implement custom error listener

        let token_factory = CommonTokenFactory::default();
        let lexer = RelationTupleLexer::new_with_token_factory(
            InputStream::new(relation_tuple.into()),
            &token_factory,
        );
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = RelationTupleParser::new(token_source);
        let parse_result = parser.relationTuple().expect("parsed unsuccessfully");
        let mut visitor = MyRelationTupleParser {
            relation_tuple_builder: Default::default(),
        };

        parse_result.accept(&mut visitor);

        visitor.relation_tuple_builder.build()
    }
}
