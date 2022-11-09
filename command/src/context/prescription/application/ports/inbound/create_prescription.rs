use crate::context::prescription::domain::entity::aggregate::PrescriptionAggregate;
use crate::context::prescription::domain::entity::command::CreatePrescriptionCommand;
use async_trait::async_trait;

#[async_trait]
pub trait CreatePrescriptionUseCase<O>
where
    O: From<PrescriptionAggregate>,
{
    async fn create_prescription(
        &self,
        prescription: CreatePrescriptionCommand,
        fields: Vec<&str>,
    ) -> Result<O, anyhow::Error>;
}
