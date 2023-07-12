from flask import Flask, request
import os
from dotenv import load_dotenv
load_dotenv()  

app = Flask(__name__)

ADDRESS_1 = "9eAsY68jFRThJdu19oB2KwaVCXC4YvsQi8kn8L9NqZWf"
ADDRESS_2 = "9eAsY68jFRThJdu19oB2KwaVCXC4YvsQi8kn8L9NqZWf"

@app.route("/pubkey", methods=["GET"])
def get_pubkey():
    return f"{ADDRESS_1},{ADDRESS_2}"

@app.route("/account", methods=["POST"])
def receive_account_update():
    data = request.get_json()
    print("Received account update:")
    print(f"Pubkey: {data['pubkey']}")
    print(f"Data: {data['data']}")
    
    return "Account update received"

if __name__ == "__main__":
    app.run(host="localhost", port=3000)
