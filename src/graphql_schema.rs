use juniper::{EmptyMutation, RootNode};

// Define queriable data -- GraphQL object(s)
struct Member {
    id: i32,
    name: String
}

#[juniper::object(description = "A member of a team")]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// GraphQL query root 
pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn members() -> Vec<Member> {
        vec![
            Member {
                id: 1,
                name: "Link".to_owned(),
            },
            Member {
                id: 2,
                name: "Mario".to_owned(),
            }
        ]
    }
}

// Expose the schema with RootNode
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}