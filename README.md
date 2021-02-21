# TODO
- pattern matching on route
- enrich method 


# TO TEST
- wrong first param type (int -> other literals, Path -> Other non lits)

# CURR PROB
okay so there's no way at runtime to take whatever is in the brackets nad put it into a tuple for
routedata - the only solution to have it be 100% compile time would be to have all route params be 
strings, 

Build a vector -> convert it to a tuple. compare the tuple types with eachother - if true, then tuple is of type T and we can add it to 

For each path param, write a custom guard that tries to convert array strings into the uuid type and 
if it can it procceeds w/ next matcher, 
https://stackoverflow.com/questions/29618700/how-to-generate-tuples-from-strings

have macro generate a PathParams struct -> 

#[post("/{u_num: uuid}/")]

PathParams {
    u_num: uuid
}

impl PathParams {
    first(val: string) -> uuid {
        return val.into()
    }

    second()
}