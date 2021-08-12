use assert_fs::TempDir;
use iapyx::Protocol;
use iapyx::WalletBackend;

use std::path::PathBuf;
use std::str::FromStr;
use vit_servicing_station_tests::common::data::parse_challenges;
use vit_servicing_station_tests::common::data::parse_funds;
use vit_servicing_station_tests::common::data::parse_proposals;
use vit_servicing_station_tests::common::data::parse_reviews;
use vit_servicing_station_tests::common::data::ExternalValidVotingTemplateGenerator;

use crate::data::{challenges_eq, funds_eq, proposals_eq, reviews_eq, vitup_setup};
use vitup::scenario::network::setup_network;
use vitup::setup::start::QuickVitBackendSettingsBuilder;

#[test]
pub fn public_vote_multiple_vote_plans() {
    let proposals_path = PathBuf::from_str("../resources/tests/example/proposals.json").unwrap();
    let challenges_path = PathBuf::from_str("../resources/tests/example/challenges.json").unwrap();
    let funds_path = PathBuf::from_str("../resources/tests/example/funds.json").unwrap();
    let reviews_path = PathBuf::from_str("../resources/tests/example/review.json").unwrap();

    let mut template_generator = ExternalValidVotingTemplateGenerator::new(
        proposals_path.clone(),
        challenges_path.clone(),
        funds_path.clone(),
        reviews_path.clone(),
    )
    .unwrap();

    let expected_proposals = parse_proposals(proposals_path).unwrap();
    let expected_challenges = parse_challenges(challenges_path).unwrap();
    let expected_funds = parse_funds(funds_path).unwrap();
    let expected_reviews = parse_reviews(reviews_path).unwrap();

    if expected_funds.len() > 1 {
        panic!("more than 1 expected fund is not supported");
    }

    let expected_fund = expected_funds.iter().next().unwrap().clone();

    let endpoint = "127.0.0.1:8080";
    let testing_directory = TempDir::new().unwrap().into_persistent();
    let mut quick_setup = QuickVitBackendSettingsBuilder::new();
    quick_setup
        .vote_start_epoch(0)
        .tally_start_epoch(1)
        .tally_end_epoch(2)
        .fund_id(expected_fund.id)
        .slot_duration_in_seconds(2)
        .slots_in_epoch_count(30)
        .proposals_count(expected_proposals.len() as u32)
        .challenges_count(expected_challenges.len())
        .voting_power(expected_fund.threshold.unwrap() as u64)
        .private(false);

    let (mut vit_controller, mut controller, vit_parameters, _) =
        vitup_setup(quick_setup, testing_directory.path().to_path_buf());
    let (nodes, vit_station, wallet_proxy) = setup_network(
        &mut controller,
        &mut vit_controller,
        vit_parameters,
        &mut template_generator,
        endpoint.to_string(),
        &Protocol::Http,
        "2.0".to_owned(),
    )
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(10));

    let backend_client = WalletBackend::new(endpoint.to_string(), Default::default());

    let actual_fund = backend_client.funds().unwrap();
    let actual_challenges = backend_client.challenges().unwrap();
    let actual_proposals = backend_client.proposals().unwrap();
    let actual_reviews = backend_client.reviews().unwrap();

    funds_eq(expected_fund, actual_fund);
    challenges_eq(expected_challenges, actual_challenges);
    proposals_eq(expected_proposals, actual_proposals);
    reviews_eq(expected_reviews, actual_reviews);

    vit_station.shutdown();
    wallet_proxy.shutdown();
    for node in nodes {
        node.shutdown().unwrap();
    }
    controller.finalize();
}
