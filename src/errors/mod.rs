/// The `HashError` enum is a collection of all the possible reasons
/// an enum can fail to parse from a string.
#[derive(Debug)]
pub enum HashError {
    AlgorithmNotFound,
    HashingError
}