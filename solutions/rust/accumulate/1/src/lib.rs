/// What should the type of _function be?
pub fn map<T, O, F>(input: Vec<T>, mut function: F) -> Vec<O> 
    where F: FnMut(T)->O {
    let mut output = Vec::with_capacity(input.len());

    for val in input {
        output.push(function(val));
    }

    output
}
