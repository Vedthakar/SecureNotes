from django.shortcuts import render
from rest_framework.generics import GenericAPIView, RetrieveAPIView
from rest_framework.permissions import AllowAny, IsAuthenticated, BasePermission
from rest_framework.response import Response
from rest_framework import status, generics, permissions
from rest_framework.decorators import api_view
from django.conf import settings
from .models import CustomNote
from .serializers import NoteSerializer
from .serializers import *
from .authTokens import create_auth_tokens, create_access_token, AuthlibJWTAuthentication
from rest_framework_simplejwt.tokens import RefreshToken
from authlib.jose import jwt, JoseError
from django.contrib.auth import get_user_model

User = get_user_model()

class UserRegistrationAPIView(GenericAPIView):
    permission_classes = (AllowAny,)
    serializer_class = UserRegistrationSerializer
    def add_ingredient(request):
        name = request.data.get('username')
        email = request.data.get('email')
        password = request.data.get('password')

        try:
            user = CustomUser.objects.get(id=name, owner=user)
            CustomUser.objects.create(
                name=name,
                email=email,
                password=password
            )
            return Response({"message": "user added"}, status=201)
        except CustomUser.DoesNotExist:
            return Response({"error": "user not found"}, status=404)

    def post(self, request, *args, **kwargs):
        try:
            serializer = self.get_serializer(data=request.data)
            serializer.is_valid(raise_exception=True)
            user = serializer.save()
            tokens = create_auth_tokens(user, settings.SECRET_KEY)
            data = serializer.data
            data['tokens'] = tokens
            return Response(data, status=status.HTTP_201_CREATED)
        except Exception as e:
            print("❌ ERROR IN REGISTRATION:", str(e))
            import traceback; traceback.print_exc()
            return Response({"error": str(e)}, status=500)


class UserLoginAPIView(GenericAPIView):
    permission_classes = (AllowAny,)
    serializer_class = UserLoginSerializer

    def post(self, request, *args, **kwargs):
        print("🔥 LOGIN REQUEST RECEIVED")
        print("REQUEST DATA:", request.data)
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)
        user = serializer.validated_data

        tokens = create_auth_tokens(user, settings.SECRET_KEY)
        user_data = CustomeUserSerializer(user).data
        user_data['tokens'] = tokens

        return Response(user_data, status=status.HTTP_200_OK)


class UserLogoutAPIView(GenericAPIView):
    permission_classes = (IsAuthenticated,)

    def post(self, request, *args, **kwargs):
        try:
            refresh_token = request.data["refresh"]
            token = RefreshToken(refresh_token)
            token.blacklist()
            return Response(status=status.HTTP_205_RESET_CONTENT)
        except Exception as e:
            return Response(status=status.HTTP_205_RESET_CONTENT)


class DebugIsAuthenticated(BasePermission):
    def has_permission(self, request, view):
        print("🧺 Debug Permission → request.user:", request.user)
        print("✅ is_authenticated:", request.user.is_authenticated)
        return request.user and request.user.is_authenticated


class UserInfoAPIView(RetrieveAPIView):
    authentication_classes = [AuthlibJWTAuthentication]
    permission_classes = [DebugIsAuthenticated]
    serializer_class = CustomeUserSerializer

    def get(self, request, *args, **kwargs):
        print("🔑 Header:", request.META.get("HTTP_AUTHORIZATION"))
        print("🔒 Received GET to /api/user/")
        print("🗾 User:", request.user)
        print("✅ Auth successful:", request.user.is_authenticated)
        return super().get(request, *args, **kwargs)

    def get_object(self):
        return self.request.user

class UserNotesAPIView(RetrieveAPIView):
    authentication_classes = [AuthlibJWTAuthentication]
    permission_classes = [DebugIsAuthenticated]

    def get(self, request, *args, **kwargs):
        user = request.user
        notes = user.notes.all()
        data = [
            {
                "id": note.id,
                "title": note.title,
                "content": note.content,
            }
            for note in notes
        ]
        return Response(data)
    
    def post(self, request, *args, **kwargs):
        title = request.data.get("title")
        content = request.data.get("content")
        user = request.user  # assumes the user is authenticated

        if not title or not content:
            return Response({"error": "Title and content required"}, status=status.HTTP_400_BAD_REQUEST)

        note = CustomNote.objects.create(
                title=title,
                content=content,
                user=user
            )

        return Response({
            "id": note.id,
            "title": note.title,
            "content": note.content,
            "user": note.user.username
        }, status=status.HTTP_200_OK)
    
class UserNoteDetailAPIView(generics.RetrieveUpdateAPIView):
    queryset = CustomNote.objects.all()
    serializer_class = NoteSerializer
    permission_classes = [permissions.IsAuthenticated]

    def get_queryset(self):
        return super().get_queryset().filter(user=self.request.user)

@api_view(['POST'])
def refresh_tokens(request):
    print("♻️ Token refresh request received...")

    refresh_token = request.data.get("refresh_token", "")
    if not refresh_token:
        return Response({"error": "Missing refresh_token"}, status=status.HTTP_400_BAD_REQUEST)

    try:
        claims = jwt.decode(refresh_token, settings.SECRET_KEY)
        print("✅ Refresh JWT decoded:", claims)
        claims.validate()
    except JoseError as e:
        print("❌ Invalid refresh token:", str(e))
        return Response({"error": "Invalid or expired refresh token"}, status=status.HTTP_401_UNAUTHORIZED)

    if claims.get("token_type") != "refresh":
        return Response({"error": "Invalid token type"}, status=status.HTTP_400_BAD_REQUEST)

    try:
        user = User.objects.get(id=claims["sub"])
        print("👤 User found for refresh:", user.username)
    except User.DoesNotExist:
        return Response({"error": "User not found"}, status=status.HTTP_404_NOT_FOUND)

    new_access_token = create_access_token(user, settings.SECRET_KEY)
    return Response({"access": new_access_token}, status=status.HTTP_200_OK)
