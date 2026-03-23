//! bug 1 - request bytes are not being read from the connection stream
//! ---
//! was due to the request stream data reader while loop condition being:
//! the length of the ascii trimmed data buffer equaling 0
//! -> which is obbiously always true at the start since we clear the buffer before reading

//! bug 2 - the request data reading loop never breaks
//! ---
//! the expected behaviour was that read line would read only 1 line from the request
//! meanwhile it ended up reading the entire request data all at once
//! which nullified the loop break condition (that the line string/bufer be empty when ascii trimmed)
//! not sure if this has true: stopped observing it suddenly, may have been me hallucinating
//!
//! bug 2 [re-evaluated] -
//! ---
//! the expected behaviour was that read line would overwrite its buffer on new reads
//! meanwhile it ended up appending the new read data to the buffer

//! bug 3 - the first client request is always not handled correctly
//! ---
//! when the request reading loop has a bad condition that would never trigger,
//! the first client request gets read and printed to stdout and stops there (despite bad loop
//! condition), but a response is not returned to the client
//! meanwhile subsequent requests trigger an infinite loop (until client closes connection, if it
//! does) of client requests being sent
//!

//! bug report
//! ---
//! need more experimentation to uncover valuable observations
//!
//! correcting the stream read loop break condition fixed all the above bugs.
//! not handling the stream reading operations properly could result in unpredictable bugs

#[test]
fn receives_request() {}
