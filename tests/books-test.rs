use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

#[test]
fn test_get_books() {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/books").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK)
}


#[test]
fn test_create_book() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "create_test",
            "description": "This book is for creating stuff"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "create_test",
        "description": "This book is for creating stuff",
        "created_at": book_response["created_at"],
    }))
}


#[test]
fn test_delete_book() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "delete_test",
            "description": "this book is for deleting stuff"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "delete_test",
        "description": "this book is for deleting stuff",
        "created_at": book_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT)
}


#[test]
fn test_get_book() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "get_book_test",
            "description": "this book is for getting stuff"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "get_book_test",
        "description": "this book is for getting stuff",
        "created_at": book_response["created_at"],
    }));

    let response = client.get(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "get_book_test",
        "description": "this book is for getting stuff",
        "created_at": book_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}


#[test]
fn test_delete_and_check_book() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "test_delete_and_check_book",
            "description": "test_delete_and_check_book"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "test_delete_and_check_book",
        "description": "test_delete_and_check_book",
        "created_at": book_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap(); 
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}   


#[test]
fn test_create_check_and_delete_book() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "test_create_check_and_delete",
            "description": "test_create_check_and_delete_book"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "test_create_check_and_delete",
        "description": "test_create_check_and_delete_book",
        "created_at": book_response["created_at"],
    }));

    let response = client.get(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "test_create_check_and_delete",
        "description": "test_create_check_and_delete_book",
        "created_at": book_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)

}


#[test]
fn test_update_book() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/books")
        .json(&json!({
            "name": "update_test",
            "description": "update_test_book",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let book_response:Value = response.json().unwrap();
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "update_test",
        "description": "update_test_book",
        "created_at": book_response["created_at"],
    }));
    
    let response = client.put("http://127.0.0.1:8000/books")
        .json(&json!({
            "id": book_response["id"],
            "name": "test_update_book_change",
            "description": "test_update_book_change@changdescription.com",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let book_response:Value = response.json().unwrap();
    print!("{:?}", book_response);
    assert_eq!(book_response, json!({
        "id": book_response["id"],
        "name": "test_update_book_change",
        "description": "test_update_book_change@changdescription.com",
        "created_at": book_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get(format!("http://127.0.0.1:8000/books/{}", book_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}