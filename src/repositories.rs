use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::*;
use crate::models::*;

pub struct MemberRepository;

impl MemberRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Member>{
        members::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i64) -> QueryResult<Vec<Member>>{
        members::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_member: NewMember) -> QueryResult<Member>{
        diesel::insert_into(members::table).values(new_member).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, member: Member) -> QueryResult<Member>{
        diesel::update(members::table.find(member.id)).set((
            members::name.eq(member.name),
            members::email.eq(member.email),
        )).get_result(conn).await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(members::table.find(id)).execute(conn).await
    }
}

pub struct BookRepository;

impl BookRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Book>{
        books::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i64) -> QueryResult<Vec<Book>>{
        books::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_book: NewBook) -> QueryResult<Book>{
        diesel::insert_into(books::table).values(new_book).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, book: Book) -> QueryResult<Book>{
        diesel::update(books::table.find(book.id)).set((
            books::name.eq(book.name),
            books::description.eq(book.description),
        )).get_result(conn).await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(books::table.find(id)).execute(conn).await
    }
}

pub struct ReviewRepository;

impl ReviewRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Review>{
        reviews::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i64) -> QueryResult<Vec<Review>>{
        reviews::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_review: NewReview) -> QueryResult<Review>{
        diesel::insert_into(reviews::table).values(new_review).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, review: Review) -> QueryResult<Review>{
        diesel::update(reviews::table.find(review.id)).set((
            reviews::review.eq(review.review),
            reviews::member_id.eq(review.member_id),
            reviews::book_id.eq(review.book_id),
        )).get_result(conn).await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(reviews::table.find(id)).execute(conn).await
    }
}

pub struct GroupRepository;

impl GroupRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Group>{
        groups::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i64) -> QueryResult<Vec<Group>>{
        groups::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_group: NewGroup) -> QueryResult<Group>{
        diesel::insert_into(groups::table).values(new_group).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, group: Group) -> QueryResult<Group>{
        diesel::update(groups::table.find(group.id)).set((
            groups::name.eq(group.name),
            groups::current_book_id.eq(group.current_book_id),
        )).get_result(conn).await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(groups::table.find(id)).execute(conn).await
    }
}

pub struct GroupMemberRepository;

impl GroupMemberRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<GroupMember>{
        group_members::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i64) -> QueryResult<Vec<GroupMember>>{
        group_members::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_group_member: NewGroupMember) -> QueryResult<GroupMember>{
        diesel::insert_into(group_members::table).values(new_group_member).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, group_member: GroupMember) -> QueryResult<GroupMember>{
        diesel::update(group_members::table.find(group_member.id)).set((
            group_members::user_id.eq(group_member.user_id),
            group_members::group_id.eq(group_member.group_id),
        )).get_result(conn).await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(group_members::table.find(id)).execute(conn).await
    }
}
pub struct ChatRepository;

impl ChatRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<Chat>{
        chats::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i64) -> QueryResult<Vec<Chat>>{
        chats::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_chat: NewChat) -> QueryResult<Chat>{
        diesel::insert_into(chats::table).values(new_chat).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, chat: Chat) -> QueryResult<Chat>{
        diesel::update(chats::table.find(chat.id)).set((
            chats::message.eq(chat.message),
            chats::member_id.eq(chat.member_id),
            chats::to_member_id.eq(chat.to_member_id),
        )).get_result(conn).await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(chats::table.find(id)).execute(conn).await
    }
}

pub struct GroupChatRepository;

impl GroupChatRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id:i32) -> QueryResult<GroupChat>{
        group_chats::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit:i64) -> QueryResult<Vec<GroupChat>>{
        group_chats::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_group_chat: NewGroupChat) -> QueryResult<GroupChat>{
        diesel::insert_into(group_chats::table).values(new_group_chat).get_result(conn).await
    }

    pub async fn update(conn: &mut AsyncPgConnection, group_chat: GroupChat) -> QueryResult<GroupChat>{
        diesel::update(group_chats::table.find(group_chat.id)).set((
            group_chats::message.eq(group_chat.message),
            group_chats::group_id.eq(group_chat.group_id),
            group_chats::to_member_id.eq(group_chat.to_member_id),
        )).get_result(conn).await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>{
        diesel::delete(group_chats::table.find(id)).execute(conn).await
    }
}