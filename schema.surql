// REFER - https://surrealdb.com/docs/introduction/concepts for knowing about the system structure
// of SurrealDB.

// Each namespace is dedicated to a microservice.
DEFINE NAMESPACE authentication_microservice;
USE NS authentication_microservice;

  // User representing the microservice.
  DEFINE USER authentication_microservice ON NAMESPACE
    PASSWORD 'password'
    ROLES EDITOR; // Can view and edit any resource in this namespace, except users or token (IAM)
                  // resources.

  DEFINE DATABASE authentication_microservice;
  USE DB authentication_microservice;

    DEFINE TABLE users SCHEMAFULL;

      DEFINE FIELD email ON TABLE users TYPE string;
      DEFINE INDEX email_index ON TABLE users FIELDS email UNIQUE;

      DEFINE FIELD username ON TABLE users TYPE string;
      DEFINE FIELD password ON TABLE users TYPE string;

      DEFINE FIELD is_verified ON TABLE users TYPE bool
        DEFAULT false;

      DEFINE FIELD verification_code ON TABLE users TYPE option<string>;

      DEFINE FIELD created_at ON TABLE users TYPE datetime
        DEFAULT time::unix( );

      // When an unconfirmed user verifies his/her email.
      DEFINE EVENT new_verified_user ON TABLE users WHEN $event="CREATE" THEN { };