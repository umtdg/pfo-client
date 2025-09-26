use anyhow::Result;

use crate::cli::args::FundCommand;
use crate::client::Client;
use crate::client::models::fund::{FundInformation, FundStats};
use crate::output::{FundInformationColumn, FundStatsColumn, OutputTable};

pub async fn handle(cmd: FundCommand, client: Client) -> Result<()> {
    match cmd {
        FundCommand::Get {
            codes,
            date,
            from,
            output,
            no_headers,
            wide,
        } => {
            let fund_infos = client.get_funds(codes, date, from).await?;
            let columns = output.unwrap_or(vec![
                FundInformationColumn::Code,
                FundInformationColumn::Title,
                FundInformationColumn::Date,
                FundInformationColumn::Price,
                FundInformationColumn::TotalValue,
            ]);
            let headers = !no_headers;

            FundInformation::print_table(&fund_infos, &columns, headers, wide);
        }
        FundCommand::Stats {
            codes,
            output,
            force,
            no_headers,
            wide,
        } => {
            let fund_stats = client.get_fund_stats(codes, force).await?;
            let columns = output.unwrap_or(vec![
                FundStatsColumn::Code,
                FundStatsColumn::LastPrice,
                FundStatsColumn::TotalValue,
                FundStatsColumn::Yearly,
                FundStatsColumn::ThreeYearly,
                FundStatsColumn::FiveYearly,
            ]);
            let headers = !no_headers;
            FundStats::print_table(&fund_stats, &columns, headers, wide);
        }
    }

    Ok(())
}
