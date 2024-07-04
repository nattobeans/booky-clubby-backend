use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

#[test]
fn test_get_groups() {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/groups").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK)
}


#[test]
fn test_create_group() {
    let client = Client::new();

    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "create group test book",
            "description": "This book testing the create endpoint of groups"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let test_book:Value = response.json().unwrap();

    let response = client.post("http://127.0.0.1:8000/groups")
        .json(&json!({
            "name": "create_test",
            "current_book_id": test_book["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "create_test",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }))
}


#[test]
fn test_delete_group() {
    let client = Client::new();

    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "delete group book",
            "description": "This book is testing the delete endpoint of groups"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let test_book:Value = response.json().unwrap();

    let response = client.post("http://127.0.0.1:8000/groups")
        .json(&json!({
            "name": "delete_test",
            "current_book_id": test_book["id"],
        }))
        . send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "delete_test",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}


#[test]
fn test_get_group() {
    let client = Client::new();

    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "get group book",
            "description": "This book is testing the get endpoint of groups"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let test_book:Value = response.json().unwrap();

    let response = client.post("http://127.0.0.1:8000/groups")
        .json(&json!({
            "name": "get_group_test",
            "current_book_id": test_book["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "get_group_test",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }));

    let response = client.get(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "get_group_test",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}


#[test]
fn test_delete_and_check_group() {
    let client = Client::new();


    let response = client.post("http://127.0.0.1:8000/books")
    .json(&json!({
        "name": "delete and check group book",
        "description": "This book is testing the delete and check endpoint of groups"
    }))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let test_book:Value = response.json().unwrap();


    let response = client.post("http://127.0.0.1:8000/groups")
        .json(&json!({
            "name": "test_delete_and_check_group",
            "current_book_id": test_book["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "test_delete_and_check_group",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap(); 
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}   


#[test]
fn test_create_check_and_delete_group() {
    let client = Client::new();

    let response = client.post("http://127.0.0.1:8000/books")
    .json(&json!({
        "name": "delete and check group book",
        "description": "This book is testing the delete and check endpoint of groups"
    }))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let test_book:Value = response.json().unwrap();


    let response = client.post("http://127.0.0.1:8000/groups")
        .json(&json!({
            "name": "test_create_check_and_delete",
            "current_book_id": test_book["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "test_create_check_and_delete",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }));

    let response = client.get(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "test_create_check_and_delete",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)

}


#[test]
fn test_update_group() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/books")
    .json(&json!({
        "name": "update group book",
        "description": "This book is testing the delete and check endpoint of groups"
    }))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let test_book:Value = response.json().unwrap();

    let response = client.post("http://127.0.0.1:8000/books")
    .json(&json!({
        "name": "update2 group book",
        "description": "This book is testing the delete and check endpoint of groups"
    }))
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let test_update_book:Value = response.json().unwrap();

    let response = client.post("http://127.0.0.1:8000/groups")
        .json(&json!({
            "name": "update_test",
            "current_book_id": test_book["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let group_response:Value = response.json().unwrap();
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "update_test",
        "current_book_id": test_book["id"],
        "created_at": group_response["created_at"],
    }));
    
    let response = client.put("http://127.0.0.1:8000/groups")
        .json(&json!({
            "id": group_response["id"],
            "name": "test_update_group_change",
            "current_book_id": test_update_book["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let group_response:Value = response.json().unwrap();
    print!("{:?}", group_response);
    assert_eq!(group_response, json!({
        "id": group_response["id"],
        "name": "test_update_group_change",
        "current_book_id": test_update_book["id"],
        "created_at": group_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/groups/{}", group_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}