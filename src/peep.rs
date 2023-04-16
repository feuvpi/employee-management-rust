//...
fn list_peeps(total_peeps: i64, conn: &DBPooledConnection) -> Result<Peeps, Error> {
    use crate::schema::peeps::dsl::*;

    let _peeps = match peeps
        .order(created_at.desc())
        .limit(total_peeps)
        .load::<PeepDB>(conn)
    {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    Ok(Peeps {
        results: _peeps
            .into_iter()
            .map(|t| t.to_peep())
            .collect::<Vec<Peep>>(),
    })
}

fn find_peep(_id: Uuid, conn: &DBPooledConnection) -> Result<Peep, Error> {
    use crate::schema::peeps::dsl::*;

    let res = peeps.filter(id.eq(_id)).load::<PeepDB>(conn);
    match res {
        Ok(peeps_db) => match peeps_db.first() {
            Some(peep_db) => Ok(peep_db.to_peep()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

fn create_peep(peep: Peep, conn: &DBPooledConnection) -> Result<Peep, Error> {
    use crate::schema::peeps::dsl::*;

    let peep_db = peep.to_peep_db();
    let _ = diesel::insert_into(peeps).values(&peep_db).execute(conn);

    Ok(peep_db.to_peep())
}

fn delete_peep(_id: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::peeps::dsl::*;

    let res = diesel::delete(peeps.filter(id.eq(_id))).execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
//...
