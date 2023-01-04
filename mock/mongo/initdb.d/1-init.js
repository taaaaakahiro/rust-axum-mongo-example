var users = [
    {
      user: "root",
      pwd: "password",
      roles: [
        {
          role: "readWrite",
          db: "example"
        }
      ]
    }
  ];
  
  for (var i = 0, length = users.length; i < length; ++i) {
    db.createUser(users[i]);
  }