from authlib.jose import jwt, JoseError
from datetime import datetime, timedelta, timezone
import os
from django.conf import settings
from django.contrib.auth import get_user_model
from rest_framework.authentication import BaseAuthentication
from rest_framework.exceptions import AuthenticationFailed


# Token creation function
def create_access_token(user, secret_key):
    now = datetime.now(timezone.utc)
    access_token_exp = now + timedelta(minutes=30)
    header = {"alg": "HS256", "typ": "JWT"}
    access_payload = {
        "sub": str(user.id),
        "iat": int(now.timestamp()),
        "exp": int(access_token_exp.timestamp()),
        "jti": os.urandom(16).hex(),
        "token_type": "access",
        "username": user.username,
        "email": user.email,
    }
    access_token = jwt.encode(header, access_payload, secret_key).decode('utf-8')
    print("🔐 New access token generated:", access_token)
    return access_token


def create_auth_tokens(user, secret_key):
    access_token = create_access_token(user, secret_key)

    now = datetime.now(timezone.utc)
    refresh_token_exp = now + timedelta(days=7)
    header = {"alg": "HS256", "typ": "JWT"}
    refresh_payload = {
        "sub": str(user.id),
        "iat": int(now.timestamp()),
        "exp": int(refresh_token_exp.timestamp()),
        "jti": os.urandom(16).hex(),
        "token_type": "refresh",
    }
    refresh_token = jwt.encode(header, refresh_payload, secret_key).decode('utf-8')
    print("🔄 Refresh token generated:", refresh_token)

    return {"access": access_token, "refresh": refresh_token}


# Custom Authlib-based authentication
User = get_user_model()

class AuthlibJWTAuthentication(BaseAuthentication):
    def authenticate(self, request):
        print("🔍 Starting AuthlibJWTAuthentication...")

        auth_header = request.META.get("HTTP_AUTHORIZATION", "")
        print("📥 Raw Auth Header:", auth_header)

        if not auth_header.startswith("Bearer "):
            print("🚫 No 'Bearer' prefix in Authorization header.")
            return None

        token = auth_header.split(" ")[1]
        print("📦 Extracted Token:", token)

        try:
            claims = jwt.decode(token, settings.SECRET_KEY)
            print("✅ JWT Decoded Claims:", claims)
            claims.validate()  # Validate 'exp', 'iat', etc.
            print("✅ Claims validated successfully")
        except JoseError as e:
            print("❌ JWT decoding/validation failed:", str(e))
            raise AuthenticationFailed(f"Invalid token: {str(e)}")

        try:
            user = User.objects.get(id=claims["sub"])
            print("👤 User found:", user.username)
        except User.DoesNotExist:
            print("❌ User ID not found in DB:", claims["sub"])
            raise AuthenticationFailed("User not found.")

        print("✅ Authentication success — returning user")
        return (user, None)
