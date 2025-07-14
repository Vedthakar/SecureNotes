import hashlib
import random
import string
from django.contrib.auth.hashers import BasePasswordHasher
from django.utils.crypto import constant_time_compare

class VedSecureHasher(BasePasswordHasher):
    algorithm = "vedhashing:"

    def salt(self):
        length = random.randint(16, 20)
        chars = string.ascii_letters + string.digits + string.punctuation
        return ''.join(random.SystemRandom().choice(chars) for _ in range(length))

    def encode(self, password, salt):
        hash_input = (salt + password).encode('utf-8')
        for _ in range(500_000):
            md5_hash = hashlib.md5(hash_input).digest()
            hash_input = hashlib.sha256(md5_hash).digest()
        final_hash = hashlib.sha256(hash_input).hexdigest()
        return f"{self.algorithm}${salt}${final_hash}"

    def verify(self, password, encoded):
        algorithm, salt, final_hash = encoded.split('$', 2)
        return constant_time_compare(self.encode(password, salt), encoded)

    def must_update(self, encoded):
        return False