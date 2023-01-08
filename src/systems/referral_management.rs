use std::{fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use rmp_serde::{Serializer};
use serenity::model::id::UserId;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Contains the data regarding a User's referral
/// * 'referrals' - How many referrals the owner user has received
/// * 'referrers' - The users who referred the owner user
pub struct ReferralData {
	referrals: u8,
	referrers: Vec<UserId>
}

// Private functions
/// Creates a Path pointing to the specified user's [ReferralData]
/// * 'user' - The user to create a Path for
fn referral_path(user: UserId) -> PathBuf {
	Path::join(Path::new("./src/data/referrals"), format!("{}.txt", user))
}

// Public Functions
/// Creates ReferralData for the specified User and wraps it in a File
/// * 'referred' - The specified user
/// * 'referrer' - The User who referred the specified user
pub fn create_referral(referred: UserId, referrer: UserId) {
	// Create referral data
	let referral = ReferralData { referrals: 1, referrers: vec![referrer] };
	// Create path for new referral
	File::create(referral_path(referred)).expect(&*format!("Failed to create {}'s referral data.", referred));
	// Serialize referral data to be stored in file.
	let mut buf = Vec::new();
	referral.serialize(&mut Serializer::new(&mut buf)).unwrap();
	// Write serialized data to file.
	let mut referral_file = File::create(referral_path(referred)).expect("Failed to write serialized referral data to file.");
	referral_file.write(&*buf).unwrap();
}

pub fn update_referral(referred: UserId, referrer: UserId) {
	let mut referral;
	if let Some(referral_data) = get_referral(referred) { referral = referral_data } else { return }
	referral.referrals += 1;
	referral.referrers.push(referrer);
}

pub fn get_referral(user: UserId) -> Option<ReferralData> {
	let referral_path = referral_path(user);
	if !Path::exists(&*referral_path) { return None }

	let mut referral_file = File::open(referral_path).unwrap();
	let mut serialized_referral = Vec::new();
	referral_file.read_to_end(&mut serialized_referral).unwrap();

	Some(ReferralData::from(rmp_serde::from_slice(&*serialized_referral).unwrap()))
}

pub fn has_referral(user: UserId) -> bool {
	Path::exists(&*referral_path(user))
}

pub fn _erase_referral(user: UserId) {
	fs::remove_file(referral_path(user)).expect(&*format!("Failed to erase {}'s referral data.", user));
}