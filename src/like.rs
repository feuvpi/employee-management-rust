pub fn list_likes(_id_peep: Uuid, conn: &DBPooledConnection) -> Result<Likes, Error> {
    use crate::schema::likes::dsl::*;

    let _likes: Vec<LikeDB> = match likes
        .filter(id_peep.eq(_id_peep))
        .order(created_at.desc())
        .load::<LikeDB>(conn)
    {
        Ok(lks) => lks,
        Err(_) => vec![],
    };

    Ok(Likes {
        results: _likes
            .into_iter()
            .map(|l| l.to_like())
            .collect::<Vec<Like>>(),
    })
}

pub fn create_like(_id_peep: Uuid, conn: &DBPooledConnection) -> Result<Like, Error> {
    use crate::schema::likes::dsl::*;

    let like = Like::new();
    let _ = diesel::insert_into(likes)
        .values(like.to_like_db(_id_peep))
        .execute(conn);

    Ok(like)
}

pub fn delete_like(_id_peep: Uuid, conn: &DBPooledConnection) -> Result<(), Error> {
    use crate::schema::likes::dsl::*;

    let _likes = list_likes(_id_peep, conn);

    let like = match &_likes {
        Ok(_likes) if !_likes.results.is_empty() => _likes.results.first(),
        _ => None,
    };

    if like.is_none() {
        return Ok(());
    }

    let like_id = Uuid::from_str(like.unwrap().id.as_str()).unwrap();

    let res = diesel::delete(likes.filter(id.eq(like_id))).execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
