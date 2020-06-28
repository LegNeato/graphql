use chrono::NaiveDate;
use juniper::{Context as JuniperContext, EmptySubscription, FieldResult, RootNode};
use std::sync::Arc;
use tokio_postgres::Client;
use uuid::Uuid;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub struct Context {
    client: Arc<Client>,
}

impl Context {
    pub fn with(client: &Arc<Client>) -> Context {
        Context {
            client: Arc::clone(client),
        }
    }
}

impl JuniperContext for Context {}

pub struct Member {
    pub id: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub birthdate: NaiveDate,
}

#[graphql_object(Context = Context)]
impl Member {
    fn id(&self) -> &str {
        &self.id
    }
    fn email(&self) -> &str {
        &self.email
    }
    fn firstname(&self) -> &str {
        &self.firstname
    }
    fn lastname(&self) -> &str {
        &self.lastname
    }
    fn birthdate(&self) -> NaiveDate {
        self.birthdate
    }
    async fn rides(&self, context: &Context) -> FieldResult<Vec<Ride>> {
        let rows = context
            .client
            .query(
                "SELECT id, name, description, distance, started, ended FROM ride WHERE rider = $1",
                &[&self.id],
            )
            .await?;

        let mut rides: Vec<Ride> = Vec::with_capacity(rows.len());

        for row in rows {
            rides.push(Ride {
                id: row.try_get(0)?,
                name: row.try_get(1)?,
                description: row.try_get(2)?,
                distance: row.try_get(3)?,
                started: row.try_get(4)?,
                ended: row.try_get(5)?,
            })
        }

        Ok(rides)
    }
}

#[derive(GraphQLObject)]
pub struct Ride {
    pub id: String,
    pub name: String,
    pub description: String,
    pub distance: i32,
    pub started: NaiveDate,
    pub ended: NaiveDate,
}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    async fn member(ctx: &Context, id: String) -> FieldResult<Member> {
        let uuid = Uuid::parse_str(&id)?;

        let row = ctx
            .client
            .query_one(
                "SELECT email, firstname, lastname, birthdate FROM member WHERE id = $1",
                &[&uuid],
            )
            .await?;

        let member = Member {
            id,
            email: row.try_get(0)?,
            firstname: row.try_get(1)?,
            lastname: row.try_get(2)?,
            birthdate: row.try_get(3)?,
        };

        Ok(member)
    }

    async fn members(ctx: &Context) -> FieldResult<Vec<Member>> {
        let rows = ctx
            .client
            .query(
                "SELECT id, email, firstname, lastname, birthdate FROM member",
                &[],
            )
            .await?;

        let mut members = Vec::new();

        for row in rows {
            let uuid: Uuid = row.try_get(0)?;

            members.push(Member {
                id: uuid.to_string(),
                email: row.try_get(1)?,
                firstname: row.try_get(2)?,
                lastname: row.try_get(3)?,
                birthdate: row.try_get(4)?,
            });
        }

        Ok(members)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn register_member(
        ctx: &Context,
        email: String,
        firstname: String,
        lastname: String,
        birthdate: NaiveDate,
    ) -> FieldResult<Member> {
        let id = Uuid::new_v4();
        let email = email.to_lowercase();

        ctx.client.execute(
            "INSERT INTO member(id, email, firstname, lastname, birthdate) VALUES ($1, $2, $3, $4, $5)",
            &[&id, &email, &firstname, &lastname, &birthdate],
        ).await?;

        Ok(Member {
            id: id.to_string(),
            email,
            firstname,
            lastname,
            birthdate,
        })
    }

    async fn register_ride(
        ctx: &Context,
        rider: String,
        name: String,
        description: String,
        started: NaiveDate,
        ended: NaiveDate,
    ) -> FieldResult<Ride> {
        let id = Uuid::new_v4();

        ctx.client.execute(
            "INSERT INTO ride(id, rider, name, description, distance, started, ended) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[&id, &rider, &name, &description, &0, &started, &ended],
        ).await?;

        Ok(Ride {
            id: id.to_string(),
            name,
            description,
            distance: 0,
            started,
            ended,
        })
    }
}
