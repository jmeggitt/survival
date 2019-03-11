use petgraph;
use bitflags::*;
use std::collections::HashMap;
use std::sync::Arc;
use amethyst::{
    assets::{Asset, Handle},
    ecs::VecStorage,
};

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct MaterialLayer {

}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Part {
    pub name: String,
    layers: Vec<MaterialLayer>,
}
impl Part {
    pub fn new(name: &str, ) -> Self {
        Self {
            name: name.to_string(),
            layers: Vec::new(),
        }
    }
}


bitflags_serial! {
    pub struct JointRelation: u8 {
        const Inside    = 1;
        const Outside   = 1 << 1;
        const Left      = 1 << 2;
        const Right     = 1 << 3;
        const Front     = 1 << 4;
        const Back      = 1 << 5;
        const Top       = 1 << 6;
        const Bottom    = 1 << 7;
    }
}

#[derive(Clone, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Joint {
    parent: u32,
    relation: JointRelation,
    depth:    u32,
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
                body.add_edge(head, brain, Joint { relation: JointRelation::Inside, ..Default::default() });

                let r_ear = body.add_node(Part::new("Right Ear"));
                body.add_edge(r_ear, head, Joint { relation: JointRelation::Right | JointRelation::Outside, ..Default::default() });

                let l_ear = body.add_node(Part::new("Left Ear"));
                body.add_edge(l_ear, head, Joint { relation: JointRelation::Left | JointRelation::Outside, ..Default::default() });

                let r_eye = body.add_node(Part::new("Right Eye"));
                body.add_edge(r_eye, head, Joint { relation: JointRelation::Right | JointRelation::Front | JointRelation::Outside, ..Default::default() });

                let l_eye = body.add_node(Part::new("Left Eye"));
                body.add_edge(l_eye, head, Joint { relation: JointRelation::Left | JointRelation::Front | JointRelation::Outside, ..Default::default() });

                let nose = body.add_node(Part::new("Nose"));
                body.add_edge(nose, head, Joint { relation: JointRelation::Front | JointRelation::Outside, ..Default::default() });

                let mouth = body.add_node(Part::new("Mouth"));
                body.add_edge(mouth, head, Joint { relation: JointRelation::Front | JointRelation::Outside, ..Default::default() });
            }
            let neck = body.add_node(Part::new("Neck"));
            body.add_edge(neck, head, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });

            let torso = body.add_node(Part::new("Torso"));
            body.add_edge(torso, neck, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });
            {
                let r_lung = body.add_node(Part::new("Right Lung"));
                body.add_edge(torso, r_lung, Joint { relation: JointRelation::Right | JointRelation::Inside, ..Default::default() });
                let l_lung = body.add_node(Part::new("Left Lung"));
                body.add_edge(torso, l_lung, Joint { relation: JointRelation::Left | JointRelation::Inside, ..Default::default() });

                let heart = body.add_node(Part::new("Heart"));
                body.add_edge(torso, heart, Joint { relation: JointRelation::Inside, ..Default::default() });

                let liver = body.add_node(Part::new("Liver"));
                body.add_edge(torso, liver, Joint { relation: JointRelation::Inside, ..Default::default() });

                let spleen = body.add_node(Part::new("Spleen"));
                body.add_edge(torso, spleen, Joint { relation: JointRelation::Inside, ..Default::default() });

                let stomach = body.add_node(Part::new("Stomach"));
                body.add_edge(torso, stomach, Joint { relation: JointRelation::Inside, ..Default::default() });

                let int = body.add_node(Part::new("Intestines"));
                body.add_edge(torso, int, Joint { relation: JointRelation::Inside, ..Default::default() });

                let r_upper_arm = body.add_node(Part::new("Right Upper Arm"));
                body.add_edge(r_upper_arm, torso, Joint { relation: JointRelation::Right | JointRelation::Outside, ..Default::default() });
                let l_upper_arm = body.add_node(Part::new("Left Upper Arm"));
                body.add_edge(l_upper_arm, torso, Joint { relation: JointRelation::Left | JointRelation::Outside, ..Default::default() });

                let r_lower_arm = body.add_node(Part::new("Right Lower Arm"));
                body.add_edge(r_lower_arm, r_upper_arm, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });
                let l_lower_arm = body.add_node(Part::new("Left Lower Arm"));
                body.add_edge(l_lower_arm, l_upper_arm, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });

                let r_hand = body.add_node(Part::new("Right Hand"));
                body.add_edge(r_hand, r_lower_arm, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });
                let l_hand = body.add_node(Part::new("Left Hand"));
                body.add_edge(l_hand, l_lower_arm, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });


                let r_thigh = body.add_node(Part::new("Right Upper Leg"));
                body.add_edge(r_thigh, torso, Joint { relation: JointRelation::Right | JointRelation::Bottom | JointRelation::Outside, ..Default::default() });
                let l_thigh = body.add_node(Part::new("Left Upper Leg"));
                body.add_edge(l_thigh, torso, Joint { relation: JointRelation::Left | JointRelation::Bottom | JointRelation::Outside, ..Default::default() });

                let r_calf = body.add_node(Part::new("Right Lower leg"));
                body.add_edge(r_calf, r_thigh, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });
                let l_calf = body.add_node(Part::new("Left Lower Leg"));
                body.add_edge(l_calf, l_thigh, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });

                let r_foot = body.add_node(Part::new("Right Foot"));
                body.add_edge(r_foot, r_calf, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });
                let l_foot = body.add_node(Part::new("Left Foot"));
                body.add_edge(l_foot, l_calf, Joint { relation: JointRelation::Bottom | JointRelation::Outside, ..Default::default() });
            }
        }
        let serialized = ron::ser::to_string_pretty(&details.parts, ron::ser::PrettyConfig {
            depth_limit: 4,
            separate_tuple_members: false,
            enumerate_arrays: false,
            ..ron::ser::PrettyConfig::default()
        }).unwrap();
        println!("{}", serialized);
        //println!("{:?}", petgraph::dot::Dot::with_config(&details.parts, &[]));


    }
}