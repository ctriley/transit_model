// Copyright (C) 2017 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published by the
// Free Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>

use super::Exporter;
use crate::{
    objects::{Line, Network},
    Model, Result,
};
use minidom::{Element, Node};

pub struct NetworkExporter<'a> {
    model: &'a Model,
}

// Publicly exposed methods
impl<'a> NetworkExporter<'a> {
    pub fn new(model: &'a Model) -> Self {
        NetworkExporter { model }
    }
    pub fn export(&self) -> Result<Vec<Element>> {
        self.model
            .networks
            .values()
            .map(|network| self.export_network(network))
            .collect()
    }
}

// Internal methods
impl<'a> NetworkExporter<'a> {
    fn export_network(&self, network: &'a Network) -> Result<Element> {
        let element_builder = Element::builder("Network")
            .attr("id", self.generate_id(network))
            .attr("version", "any");
        let element_builder = element_builder.append(self.generate_name(network));
        let line_ref_elements = self
            .model
            .lines
            .values()
            .filter(|line| line.network_id == network.id)
            .map(|line| self.generate_line_ref(line));
        let element_builder = element_builder.append(Exporter::create_members(line_ref_elements));
        Ok(element_builder.build())
    }

    fn generate_id(&self, network: &'a Network) -> String {
        let id = network.id.replace(':', "_");
        format!("FR:network:{}:", id)
    }

    fn generate_name(&self, network: &'a Network) -> Element {
        Element::builder("Name")
            .append(Node::Text(network.name.to_owned()))
            .build()
    }

    fn generate_line_ref(&self, line: &'a Line) -> Element {
        let id = line.id.replace(':', "_");
        let line_id = format!("FR:line:{}", id);
        Element::builder("LineRef").attr("ref", line_id).build()
    }
}
