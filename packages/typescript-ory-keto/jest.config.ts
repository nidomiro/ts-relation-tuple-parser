/* eslint-disable */
export default {
  displayName: 'typescript-ory-keto',
  preset: '../../jest.preset.cjs',
  globals: {},
  testEnvironment: 'node',
  transform: {
    '^.+\\.[tj]s$': [
      'ts-jest',
      {
        tsconfig: '<rootDir>/tsconfig.spec.json',
      },
    ],
  },
  moduleFileExtensions: ['ts', 'js', 'html'],
  coverageDirectory: '../../coverage/packages/typescript-ory-keto',
}
