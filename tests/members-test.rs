use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

#[test]
fn test_get_members() {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/members").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK)
}


#[test]
fn test_create_member() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/members")
        .json(&json!({
            "name": "create_test",
            "email": "create_test@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "create_test",
        "email": "create_test@gmail.com",
        "created_at": member_response["created_at"],
    }))
}


#[test]
fn test_delete_member() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/members")
        .json(&json!({
            "name": "delete_test",
            "email": "delete_test@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "delete_test",
        "email": "delete_test@gmail.com",
        "created_at": member_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK)
}


#[test]
fn test_get_member() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/members")
        .json(&json!({
            "name": "get_member_test",
            "email": "get_member_test@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "get_member_test",
        "email": "get_member_test@gmail.com",
        "created_at": member_response["created_at"],
    }));

    let response = client.get(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "get_member_test",
        "email": "get_member_test@gmail.com",
        "created_at": member_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = client.get(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}


#[test]
fn test_create_check_and_delete_member() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/members")
        .json(&json!({
            "name": "create_test",
            "email": "create_test@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    
    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "create_test",
        "email": "create_test@gmail.com",
        "created_at": member_response["created_at"],
    }));

    let response = client.get(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "get_member_test",
        "email": "get_member_test@gmail.com",
        "created_at": member_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = client.get(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)

}


#[test]
fn test_delete_and_check_member() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/members")
        .json(&json!({
            "name": "delete_test",
            "email": "delete_test@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "delete_test",
        "email": "delete_test@gmail.com",
        "created_at": member_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = client.get(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap(); 
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR)
}   


#[test]
fn test_update_member() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/members")
        .json(&json!({
            "name": "delete_test",
            "email": "delete_test@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let member_response:Value = response.json().unwrap();
    assert_eq!(member_response, json!({
        "id": member_response["id"],
        "name": "delete_test",
        "email": "delete_test@gmail.com",
        "created_at": member_response["created_at"],
    }));

    let response = client.delete(format!("http://127.0.0.1:8000/members/{}", member_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}