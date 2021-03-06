use super::Client;
use crate::requests::RequestType;
use crate::responses;
use crate::Result;

/// API functions related to outputs.
pub struct Outputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Outputs<'a> {
    /// List existing outputs.
    pub async fn list_outputs(&self) -> Result<Vec<responses::Output>> {
        self.client
            .send_message::<responses::Outputs>(RequestType::ListOutputs)
            .await
            .map(|o| o.outputs)
    }

    /// Get information about a single output.
    ///
    /// - `output_name`: Output name.
    pub async fn get_output_info(&self, output_name: &str) -> Result<responses::Output> {
        self.client
            .send_message::<responses::OutputInfo>(RequestType::GetOutputInfo { output_name })
            .await
            .map(|o| o.output_info)
    }

    /// Start an output.
    ///
    /// - `output_name`: Output name.
    pub async fn start_output(&self, output_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::StartOutput { output_name })
            .await
    }

    /// Stop an output.
    ///
    /// - `output_name`: Output name.
    /// - `force`: Force stop (default: false).
    pub async fn stop_output(&self, output_name: &str, force: Option<bool>) -> Result<()> {
        self.client
            .send_message(RequestType::StopOutput { output_name, force })
            .await
    }
}
