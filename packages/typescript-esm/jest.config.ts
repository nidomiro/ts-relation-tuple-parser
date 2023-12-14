/* eslint-disable */
export default {
  displayName: 'typescript-esm',
  preset: '../../jest.preset.cjs',
  testEnvironment: 'node',
  transform: {
    '^.+\\.[tjm]s$': ['ts-jest', { tsconfig: '<rootDir>/tsconfig.spec.json', useESM: true }],
  },
  moduleFileExtensions: ['ts', 'js', 'mjs'],
  extensionsToTreatAsEsm: ['.ts'],
  coverageDirectory: '../../coverage/packages/typescript-esm',
}
