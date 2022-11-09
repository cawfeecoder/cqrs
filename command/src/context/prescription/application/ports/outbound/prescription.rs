pub trait PrescriptionServices: Debug {}

use std::fmt::Debug;

use mockall::mock;

mock! {
    #[derive(Debug)]
    pub PrescriptionServices {}

    impl PrescriptionServices for PrescriptionServices {}
}
