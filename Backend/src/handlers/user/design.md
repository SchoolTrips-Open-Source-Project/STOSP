User table - 
    Once otp is verified we have to create the user.
    We are going to use JWT using mobile number and  unique API key with expiry of one month.

User table will consist of 
    name : Varchar(255)
    created_at : TimeStampz
    updated_at : TimeStampz
    sessionToken : Varchar
    mobileNumber : Varchar
    role : varchar

JWT - 
{
    mobileNumber :: String,
    session_expiry_time :: UTC,
    id:: String
}