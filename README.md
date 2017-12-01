# error-test-sinks

`Sink` implementations for testing that simulate different types of errors.

### Sinks:
- `StartSendErrSink`: returns an error on `start_send`
- `PollCompleteErrSink`: returns an error on `poll_complete`
- `CloseErrSink`: returns an error on `close`

License: MIT/Apache-2.0
