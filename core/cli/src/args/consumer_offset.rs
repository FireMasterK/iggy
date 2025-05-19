/* Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use clap::{Args, Subcommand, ValueEnum};
use iggy::prelude::{ConsumerKind, Identifier};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum ConsumerType {
    /// Regular consumer
    Consumer,
    /// Consumer group
    Group,
}

impl From<ConsumerType> for ConsumerKind {
    fn from(consumer_type: ConsumerType) -> Self {
        match consumer_type {
            ConsumerType::Consumer => ConsumerKind::Consumer,
            ConsumerType::Group => ConsumerKind::ConsumerGroup,
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum ConsumerOffsetAction {
    /// Retrieve the offset of a consumer or consumer group for a given stream and topic from the server
    ///
    /// Consumer ID can be specified as a consumer name or ID
    /// Stream ID can be specified as a stream name or ID
    /// Topic ID can be specified as a topic name or ID
    /// Partition ID is required for regular consumers, optional for consumer groups
    /// Use --kind group to work with consumer groups
    ///
    /// Examples:
    ///  iggy consumer-offset get 1 3 5 1
    ///  iggy consumer-offset get consumer stream 5 1
    ///  iggy consumer-offset get 1 3 topic 1
    ///  iggy consumer-offset get consumer stream 5 1
    ///  iggy consumer-offset get consumer 3 topic 1
    ///  iggy consumer-offset get 1 stream topic 1
    ///  iggy consumer-offset get consumer stream topic 1
    ///  iggy consumer-offset get --kind group 1 3 5
    ///  iggy consumer-offset get --kind group group-name stream-name topic-name
    ///  iggy consumer-offset get --kind group group-name stream-name topic-name 1
    #[clap(verbatim_doc_comment, visible_alias = "g")]
    Get(ConsumerOffsetGetArgs),
    /// Set the offset of a consumer or consumer group for a given stream and topic on the server
    ///
    /// Consumer ID can be specified as a consumer name or ID
    /// Stream ID can be specified as a stream name or ID
    /// Topic ID can be specified as a topic name or ID
    /// Partition ID is required for regular consumers, optional for consumer groups
    /// Use --kind group to work with consumer groups
    ///
    /// Examples:
    ///  iggy consumer-offset set 1 3 5 1 100
    ///  iggy consumer-offset set consumer 3 5 1 100
    ///  iggy consumer-offset set 1 stream 5 1 100
    ///  iggy consumer-offset set 1 3 topic 1 100
    ///  iggy consumer-offset set consumer stream 5 1 100
    ///  iggy consumer-offset set consumer 3 topic 1 100
    ///  iggy consumer-offset set 1 stream topic 1 100
    ///  iggy consumer-offset set consumer stream topic 1 100
    ///  iggy consumer-offset set --kind group 1 3 5 100
    ///  iggy consumer-offset set --kind group group-name stream-name topic-name 100
    ///  iggy consumer-offset set --kind group group-name stream-name topic-name 1 100
    #[clap(verbatim_doc_comment, visible_alias = "s")]
    Set(ConsumerOffsetSetArgs),
}

#[derive(Debug, Clone, Args)]
pub(crate) struct ConsumerOffsetGetArgs {
    /// Type of consumer
    #[arg(long, value_enum)]
    pub(crate) kind: Option<ConsumerType>,

    /// Consumer or consumer group ID for which the offset is retrieved
    ///
    /// Consumer ID can be specified as a consumer name or ID
    #[clap(verbatim_doc_comment)]
    #[arg(value_parser = clap::value_parser!(Identifier))]
    pub(crate) consumer_id: Identifier,

    /// Stream ID for which consumer offset is retrieved
    ///
    /// Stream ID can be specified as a stream name or ID
    #[arg(value_parser = clap::value_parser!(Identifier))]
    pub(crate) stream_id: Identifier,

    /// Topic ID for which consumer offset is retrieved
    ///
    /// Topic ID can be specified as a topic name or ID
    #[arg(value_parser = clap::value_parser!(Identifier))]
    pub(crate) topic_id: Identifier,

    /// Partition ID for which consumer offset is retrieved
    /// Required for regular consumers, optional for consumer groups
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub(crate) partition_id: Option<u32>,
}

#[derive(Debug, Clone, Args)]
pub(crate) struct ConsumerOffsetSetArgs {
    /// Type of consumer
    #[arg(long, value_enum)]
    pub(crate) kind: Option<ConsumerType>,

    /// Consumer or consumer group ID for which the offset is set
    ///
    /// Consumer ID can be specified as a consumer name or ID
    #[clap(verbatim_doc_comment)]
    #[arg(value_parser = clap::value_parser!(Identifier))]
    pub(crate) consumer_id: Identifier,

    /// Stream ID for which consumer offset is set
    ///
    /// Stream ID can be specified as a stream name or ID
    #[arg(value_parser = clap::value_parser!(Identifier))]
    pub(crate) stream_id: Identifier,

    /// Topic ID for which consumer offset is set
    ///
    /// Topic ID can be specified as a topic name or ID
    #[arg(value_parser = clap::value_parser!(Identifier))]
    pub(crate) topic_id: Identifier,

    /// Partition ID for which consumer offset is set
    /// Required for regular consumers, optional for consumer groups
    #[arg(value_parser = clap::value_parser!(u32).range(1..))]
    pub(crate) partition_id: Option<u32>,

    /// Offset to set
    pub(crate) offset: u64,
}
