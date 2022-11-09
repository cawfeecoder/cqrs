use crate::context::prescription::domain::entity::aggregate::PrescriptionAggregate;
use crate::context::prescription::domain::entity::command::UpdatePrescriptionCommand;
use async_trait::async_trait;

#[async_trait]
pub trait UpdatePrescriptionUseCase<O>
where
    O: From<PrescriptionAggregate>,
{
    async fn update_prescription(
        &self,
        prescription: UpdatePrescriptionCommand,
        fields: Vec<&str>,
    ) -> Result<O, anyhow::Error>;
}
