from django.urls import path
from . import views
from rest_framework_simplejwt.views import TokenRefreshView
from .views import UserNotesAPIView, UserNoteDetailAPIView


urlpatterns = [
    path('register/', views.UserRegistrationAPIView.as_view(), name="register-user"),
    path('login/', views.UserLoginAPIView.as_view(), name="login-user"),
    path('logout/', views.UserLogoutAPIView.as_view(), name="logout-user"),
    path("token/refresh/", TokenRefreshView.as_view, name="token-refresh"),
    path("user/", views.UserInfoAPIView.as_view(), name="user-info"),
    path('refresh/', views.refresh_tokens, name='token_refresh'),
    path('notes/',   views.UserNotesAPIView.as_view(),       name='notes-list'),
    path('update/<int:pk>/', views.UserNoteDetailAPIView.as_view(), name='notes-detail'),
]