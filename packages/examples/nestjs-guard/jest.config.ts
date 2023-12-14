/* eslint-disable */
export default {
  displayName: 'examples-nestjs-guard',
  preset: '../../../jest.preset.cjs',
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
  coverageDirectory: '../../../coverage/packages/examples/nestjs-guard',
}
