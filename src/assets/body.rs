use std::collections::HashMap;
use std::sync::Arc;

use amethyst::{
    assets::{Asset, Handle},
    ecs::VecStorage,
};
use bitflags::*;
use petgraph;

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct MaterialLayer;

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Part {
    pub name: String,
    layers: Vec<MaterialLayer>,
}

impl Part {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            layers: Vec::new(),
        }
    }
}

bitflags_serial! {
    pub struct JointRelation: u8 {
        const INSIDE    = 1;
        const OUTSIDE   = 1 << 1;
        const LEFT      = 1 << 2;
        const RIGHT     = 1 << 3;
        const FRONT     = 1 << 4;
        const BACK      = 1 << 5;
        const TOP       = 1 << 6;
        const BOTTOM    = 1 << 7;
    }
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Joint {
    parent: u32,
    relation: JointRelation,
    depth: u32,
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Details {
    pub parts: petgraph::Graph<Part, Joint>,
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Storage {
    tag: u32,
    bodies: HashMap<String, Arc<Details>>,
}

impl Asset for Storage {
    const NAME: &'static str = "survival::Body";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_part_graph_serialization_test() {
        let mut details = Details::default();
        {
            let body = &mut details.parts;

            let head = body.add_node(Part::new("Head"));
            {
                let brain = body.add_node(Part::new("Brain"));
                body.add_edge(
                    head,
                    brain,
                    Joint {
                        relation: JointRelation::INSIDE,
                        ..Default::default()
                    },
                );

                let r_ear = body.add_node(Part::new("RIGHT Ear"));
                body.add_edge(
                    r_ear,
                    head,
                    Joint {
                        relation: JointRelation::RIGHT | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let l_ear = body.add_node(Part::new("LEFT Ear"));
                body.add_edge(
                    l_ear,
                    head,
                    Joint {
                        relation: JointRelation::LEFT | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let r_eye = body.add_node(Part::new("RIGHT Eye"));
                body.add_edge(
                    r_eye,
                    head,
                    Joint {
                        relation: JointRelation::RIGHT
                            | JointRelation::FRONT
                            | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let l_eye = body.add_node(Part::new("LEFT Eye"));
                body.add_edge(
                    l_eye,
                    head,
                    Joint {
                        relation: JointRelation::LEFT
                            | JointRelation::FRONT
                            | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let nose = body.add_node(Part::new("Nose"));
                body.add_edge(
                    nose,
                    head,
                    Joint {
                        relation: JointRelation::FRONT | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let mouth = body.add_node(Part::new("Mouth"));
                body.add_edge(
                    mouth,
                    head,
                    Joint {
                        relation: JointRelation::FRONT | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
            }
            let neck = body.add_node(Part::new("Neck"));
            body.add_edge(
                neck,
                head,
                Joint {
                    relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                    ..Default::default()
                },
            );

            let torso = body.add_node(Part::new("Torso"));
            body.add_edge(
                torso,
                neck,
                Joint {
                    relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                    ..Default::default()
                },
            );
            {
                let r_lung = body.add_node(Part::new("RIGHT Lung"));
                body.add_edge(
                    torso,
                    r_lung,
                    Joint {
                        relation: JointRelation::RIGHT | JointRelation::INSIDE,
                        ..Default::default()
                    },
                );
                let l_lung = body.add_node(Part::new("LEFT Lung"));
                body.add_edge(
                    torso,
                    l_lung,
                    Joint {
                        relation: JointRelation::LEFT | JointRelation::INSIDE,
                        ..Default::default()
                    },
                );

                let heart = body.add_node(Part::new("Heart"));
                body.add_edge(
                    torso,
                    heart,
                    Joint {
                        relation: JointRelation::INSIDE,
                        ..Default::default()
                    },
                );

                let liver = body.add_node(Part::new("Liver"));
                body.add_edge(
                    torso,
                    liver,
                    Joint {
                        relation: JointRelation::INSIDE,
                        ..Default::default()
                    },
                );

                let spleen = body.add_node(Part::new("Spleen"));
                body.add_edge(
                    torso,
                    spleen,
                    Joint {
                        relation: JointRelation::INSIDE,
                        ..Default::default()
                    },
                );

                let stomach = body.add_node(Part::new("Stomach"));
                body.add_edge(
                    torso,
                    stomach,
                    Joint {
                        relation: JointRelation::INSIDE,
                        ..Default::default()
                    },
                );

                let int = body.add_node(Part::new("Intestines"));
                body.add_edge(
                    torso,
                    int,
                    Joint {
                        relation: JointRelation::INSIDE,
                        ..Default::default()
                    },
                );

                let r_upper_arm = body.add_node(Part::new("RIGHT Upper Arm"));
                body.add_edge(
                    r_upper_arm,
                    torso,
                    Joint {
                        relation: JointRelation::RIGHT | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
                let l_upper_arm = body.add_node(Part::new("LEFT Upper Arm"));
                body.add_edge(
                    l_upper_arm,
                    torso,
                    Joint {
                        relation: JointRelation::LEFT | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let r_lower_arm = body.add_node(Part::new("RIGHT Lower Arm"));
                body.add_edge(
                    r_lower_arm,
                    r_upper_arm,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
                let l_lower_arm = body.add_node(Part::new("LEFT Lower Arm"));
                body.add_edge(
                    l_lower_arm,
                    l_upper_arm,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let r_hand = body.add_node(Part::new("RIGHT Hand"));
                body.add_edge(
                    r_hand,
                    r_lower_arm,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
                let l_hand = body.add_node(Part::new("LEFT Hand"));
                body.add_edge(
                    l_hand,
                    l_lower_arm,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let r_thigh = body.add_node(Part::new("RIGHT Upper Leg"));
                body.add_edge(
                    r_thigh,
                    torso,
                    Joint {
                        relation: JointRelation::RIGHT
                            | JointRelation::BOTTOM
                            | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
                let l_thigh = body.add_node(Part::new("LEFT Upper Leg"));
                body.add_edge(
                    l_thigh,
                    torso,
                    Joint {
                        relation: JointRelation::LEFT
                            | JointRelation::BOTTOM
                            | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let r_calf = body.add_node(Part::new("RIGHT Lower leg"));
                body.add_edge(
                    r_calf,
                    r_thigh,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
                let l_calf = body.add_node(Part::new("LEFT Lower Leg"));
                body.add_edge(
                    l_calf,
                    l_thigh,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );

                let r_foot = body.add_node(Part::new("RIGHT Foot"));
                body.add_edge(
                    r_foot,
                    r_calf,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
                let l_foot = body.add_node(Part::new("LEFT Foot"));
                body.add_edge(
                    l_foot,
                    l_calf,
                    Joint {
                        relation: JointRelation::BOTTOM | JointRelation::OUTSIDE,
                        ..Default::default()
                    },
                );
            }
        }
        let serialized = ron::ser::to_string_pretty(
            &details.parts,
            ron::ser::PrettyConfig {
                depth_limit: 4,
                separate_tuple_members: false,
                enumerate_arrays: false,
                ..ron::ser::PrettyConfig::default()
            },
        )
        .unwrap();
        println!("{}", serialized);
        //println!("{:?}", petgraph::dot::Dot::with_config(&details.parts, &[]));
    }
}
