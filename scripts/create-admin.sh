#!/bin/bash

if [ -z "$DATABASE_URL" ]; then
    echo "Need to set $DATABASE_URL"
    exit 1
fi

function gen_pwd {
    enc_pass=$( echo -n "$1" | openssl rand -base64 32 )
}

echo "### Creating admin user ###"
echo -n "Insert your email: "
read email
echo -n "Insert your pass: "
read pass

gen_pwd pass

# echo "Email is $email, pass is $pass, enc_pass is $enc_pass"
sqlite3 $DATABASE_URL "DELETE from users;"
sqlite3 $DATABASE_URL "DELETE from roles;"
sqlite3 $DATABASE_URL "INSERT INTO users (password,email,is_active) VALUES ('$enc_pass', '$email', 1);"
sqlite3 $DATABASE_URL "INSERT INTO roles (name,user) VALUES ('admin', 1);"

exit 0
