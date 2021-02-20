pub trait IntoRoute<T, Args = ()> {
    type Result;

    fn invoke(&self, receiver: &T, args: Args) -> Self::Result;
}

// ALREADY NATIVE WAY:
// have args tuple be generic over items that implement 
// IntoRouteParam type -> then, write macro for each # of input route param
// and "wire" to correct place
pub 