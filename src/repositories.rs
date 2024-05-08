use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::*;
use crate::models::*;

pub struct MemberRepository;

impl MemberRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Member>{
        members::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i32) -> QueryResult<Vec<Member>>{
        members::table.limit(limit).get_result().await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_member: NewMember) -> QueryResult<Member>{
        diesel::insert_into(members::table).values(new_member).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, member: Member) -> QueryResult<Member>{
        diesel::update(members::table.find(member.id)).set((
            members::name.eq(member.name),
            members::email.ew(member.email),
        ))
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(members::table.find(id)).execute(conn).await;
    }
}

pub struct BookRepository;

impl BookRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Book>{
        books::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i32) -> QueryResult<Vec<Book>>{
        books::table.limit(limit).get_result().await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_book: NewBook) -> QueryResult<Book>{
        diesel::insert_into(books::table).values(new_book).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, book: Book) -> QueryResult<Book>{
        diesel::update(books::table.find(book.id)).set((
            books::name.eq(book.name),
            books::email.ew(book.email),
        ))
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(books::table.find(id)).execute(conn).await;
    }
}

pub struct ReviewRepository;

impl ReviewRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Review>{
        reviews::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i32) -> QueryResult<Vec<Review>>{
        reviews::table.limit(limit).get_result().await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_review: NewReview) -> QueryResult<Review>{
        diesel::insert_into(reviews::table).values(new_review).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, review: Review) -> QueryResult<Review>{
        diesel::update(reviews::table.find(review.id)).set((
            reviews::name.eq(review.name),
            reviews::email.ew(review.email),
        ))
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(reviews::table.find(id)).execute(conn).await;
    }
}

pub struct GroupRepository;

impl GroupRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Group>{
        groups::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i32) -> QueryResult<Vec<Group>>{
        groups::table.limit(limit).get_result().await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_group: NewGroup) -> QueryResult<Group>{
        diesel::insert_into(groups::table).values(new_group).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, group: Group) -> QueryResult<Group>{
        diesel::update(groups::table.find(group.id)).set((
            groups::name.eq(group.name),
            groups::email.ew(group.email),
        ))
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(groups::table.find(id)).execute(conn).await;
    }
}

pub struct GroupMemberRepository;

impl GroupMemberRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<GroupMember>{
        group_members::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i32) -> QueryResult<Vec<GroupMember>>{
        group_members::table.limit(limit).get_result().await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_group_member: NewGroupMember) -> QueryResult<GroupMember>{
        diesel::insert_into(group_members::table).values(new_group_member).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, group_member: GroupMember) -> QueryResult<GroupMember>{
        diesel::update(group_members::table.find(group_member.id)).set((
            group_members::name.eq(group_member.name),
            group_members::email.ew(group_member.email),
        ))
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(group_members::table.find(id)).execute(conn).await;
    }
}

pub struct GroupChatRepository;

impl GroupChatRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<GroupChat>{
        group_chats::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i32) -> QueryResult<Vec<GroupChat>>{
        group_chats::table.limit(limit).get_result().await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_group_chat: NewGroupChat) -> QueryResult<GroupChat>{
        diesel::insert_into(group_chats::table).values(new_group_chat).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, group_chat: GroupChat) -> QueryResult<GroupChat>{
        diesel::update(group_chats::table.find(group_chat.id)).set((
            group_chats::name.eq(group_chat.name),
            group_chats::email.ew(group_chat.email),
        ))
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(group_chats::table.find(id)).execute(conn).await;
    }
}

pub struct ChatRepository;

impl ChatRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Chat>{
        chats::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i32) -> QueryResult<Vec<Chat>>{
        chats::table.limit(limit).get_result().await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_chat: NewChat) -> QueryResult<Chat>{
        diesel::insert_into(chats::table).values(new_chat).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, chat: Chat) -> QueryResult<Chat>{
        diesel::update(chats::table.find(chat.id)).set((
            chats::name.eq(chat.name),
            chats::email.ew(chat.email),
        ))
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(chats::table.find(id)).execute(conn).await;
    }
}