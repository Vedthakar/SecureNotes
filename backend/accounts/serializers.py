from .models import CustomUser
from rest_framework import serializers
from django.contrib.auth import authenticate
from .models import CustomNote

class CustomeUserSerializer(serializers.ModelSerializer):
    class Meta:
        model = CustomUser
        fields = ['id', 'username', 'email']

class UserRegistrationSerializer(serializers.Serializer):
    username = serializers.CharField()
    email = serializers.EmailField()
    password1 = serializers.CharField(write_only=True)
    password2 = serializers.CharField(write_only=True)

    def validate(self, attrs):
        if attrs['password1'] != attrs['password2']:
            raise serializers.ValidationError("Passwords do not match")
        if len(attrs['password1']) < 8:
            raise serializers.ValidationError("Password too short, must be at least 8 characters")
        return attrs

    def create(self, validated_data):
        password = validated_data.pop('password1')
        validated_data.pop('password2')
        return CustomUser.objects.create_user(password=password, **validated_data)


class UserLoginSerializer(serializers.Serializer):
    email = serializers.CharField()
    password = serializers.CharField(write_only = True)

    def validate(self, data):
        user = authenticate(**data)
        if user and user.is_active:
            return user
        raise serializers.ValidationError("Error")
    
class NoteSerializer(serializers.ModelSerializer):
    class Meta:
        model = CustomNote
        fields = ['id', 'title', 'content']
