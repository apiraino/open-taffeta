#!/bin/bash

if [ -z "$DATABASE_URL" ]; then
    echo "Need to set $DATABASE_URL"
    exit 1
fi

function gen_pwd {
    echo "$1"
    enc_pass=$( openssl rand -base64 32 )
}

echo "### Creating admin user ###"
echo -n "Insert your email: "
read email
echo -n "Insert your pass: "
read pass

gen_pwd pass

# echo "Email is $email, pass is $pass, enc_pass is $enc_pass"
sqlite3 $DATABASE_URL "INSERT INTO users (password,email,is_active) VALUES ('$enc_pass', '$email', 1);"

exit 0
