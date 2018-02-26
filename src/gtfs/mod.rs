// Copyright 2017-2018 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

mod read;

use std::path;
use {Collections, PtObjects};
use std::fs::File;
use gtfs::read::Config;
use Result;
use common_format::manage_calendars;

extern crate serde_json;

pub fn read<P: AsRef<path::Path>>(path: P, config_path: Option<P>) -> Result<PtObjects> {
    let mut collections = Collections::default();
    let mut contributor_as_prefix = None;

    if let Some(config_path) = config_path {
        let json_config_file = File::open(config_path)?;
        let config: Config = serde_json::from_reader(json_config_file)?;
        contributor_as_prefix = Some(config.contributor.id.clone() + ":");
        info!("config loaded: {:#?}", config);
        let (contributors, datasets) = read::read_config(config);
        collections.contributors = contributors;
        collections.datasets = datasets;
    }

    let path = path.as_ref();
    let (networks, companies) = read::read_agency(path, &contributor_as_prefix);
    collections.networks = networks;
    collections.companies = companies;
    let (stopareas, stoppoints) = read::read_stops(path, &contributor_as_prefix);
    collections.stop_areas = stopareas;
    collections.stop_points = stoppoints;
    manage_calendars(&mut collections, path)?;
    read::read_routes(path, &mut collections);
    Ok(PtObjects::new(collections)?)
}