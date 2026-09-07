# The Config Manager

## TODO

### Existing TODOs
- SSH. Use openssh. I want to reuse sessions.
- Annotate the files with SDPX using REUSE. I want to be strict with license.

### Testability Improvements

#### Refactor 1: Extract Command Execution Interface
- [ ] Create a `CommandExecutor` trait that abstracts system command execution
- [ ] Move command construction logic into separate builder/factory pattern
- [ ] Allow injection of custom command executors for testing
- [ ] Separate command preparation from execution to enable better mocking

#### Refactor 2: Implement Mock Command Runner
- [ ] Create `MockCmdRunner` implementation for testing
- [ ] Add configurable response mapping (command -> expected output)
- [ ] Implement failure simulation for error path testing
- [ ] Add assertion helpers to verify command execution patterns
- [ ] Create test fixtures with predefined command responses

#### Refactor 3: Separate Business Logic from I/O Operations
- [ ] Extract file validation logic from `ShFile::claim()` into pure functions
- [ ] Create separate `FileValidator` component that can work with test data
- [ ] Implement `ContentProcessor` for handling file content operations
- [ ] Add dependency injection for all I/O operations in `ShFile`
- [ ] Create integration test layer that can run against real or mock backends

### Testing Infrastructure
- [ ] Add unit tests for business logic components
- [ ] Create integration tests with mock command runner
- [ ] Add property-based tests for file operations
- [ ] Implement test utilities for common scenarios
- [ ] Add CI/CD pipeline with automated testing

