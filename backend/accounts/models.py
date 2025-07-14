from django.db import models
from django.contrib.auth.models import AbstractUser

class CustomUser(AbstractUser):
    email = models.EmailField(unique=True)
    USERNAME_FIELD = 'email'
    REQUIRED_FIELDS = ['username']

    def __str__(self) -> str:
        return self.email
class CustomNote(models.Model):
    title = models.TextField()
    content = models.TextField()
    note_type = models.TextField()
    user = models.ForeignKey(CustomUser,on_delete=models.CASCADE, related_name='notes')

    def __str__(self) -> str:
        return self.title
