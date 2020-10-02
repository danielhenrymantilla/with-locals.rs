/// To avoid a bug when cross compiling
extern crate proc_macros;

pub use ::proc_macros::with;

#[doc(hidden)] /** Not part of the public API **/ pub
mod __internals__ {
    pub
    enum ControlFlow<Eval, Error, Return, Break> {
        /// Classic block evaluation.
        Eval(Eval),

        /// Must `return` the value early.
        EarlyReturn(Return),

        /// Must propagate the encountered error.
        ///
        /// Cannot be wrapped within the previous variant because in order to
        /// support `None`s, we need to apply to this returned value
        /// `Try::from_err`.
        PropagateError(Error),

        /// Must `break` with the value.
        Break(Break),

        /// Must `continue`.
        Continue,
    }

    /// Custom *void type*, to generate stuff like `ControlFlow<_, __, _, __>`.
    pub
    enum __ {}

    pub
    use ::core;

    pub
    trait Try {
        type Ok;
        type Error;

        fn into_result (self: Self)
          -> Result<Self::Ok, Self::Error>
        ;

        fn from_ok (ok: Self::Ok)
          -> Self
        ;

        fn from_err (err: Self::Error)
          -> Self
        ;
    }

    impl<Ok, Err> Try for Result<Ok, Err> {
        type Ok = Ok;
        type Error = Err;

        #[inline]
        fn into_result (self: Result<Ok, Err>)
          -> Result<Ok, Err>
        {
            self
        }

        #[inline]
        fn from_ok (ok: Ok)
          -> Result<Ok, Err>
        {
            Ok(ok)
        }

        #[inline]
        fn from_err (err: Err)
          -> Result<Ok, Err>
        {
            Err(err)
        }
    }

    pub struct NoneError;

    impl<T> Try for Option<T> {
        type Ok = T;
        type Error = NoneError;

        #[inline]
        fn into_result (self: Option<T>)
          -> Result<T, NoneError>
        {
            self.ok_or(NoneError)
        }

        #[inline]
        fn from_ok (value: T)
          -> Option<T>
        {
            Some(value)
        }

        #[inline]
        fn from_err (NoneError: NoneError)
          -> Option<T>
        {
            None
        }
    }
}
