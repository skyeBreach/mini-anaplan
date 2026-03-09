databaseName = "mini-polaris";

db = db.getSiblingDB("admin");
db.auth("mongo_root", "example");

db = db.getSiblingDB(databaseName);

db.createUser({
    user: "mongo_user",
    pwd: "password",
    roles: [
        {
            role: "readWrite",
            db: databaseName,
        },
    ],
});

db.createCollection("users");

db.users.insert({
    id: 1,
    username: "admin",
    password: "admin",
    first_name: "Admin",
    last_name: "Admin",
});

db.users.insert({
    id: 2,
    username: "johndoe",
    password: "password",
    first_name: "John",
    last_name: "Doe",
});

db.createCollection("tokens");
