from flask import Flask, request

app = Flask(__name__)

@app.route("/pubkey", methods=["GET"])
def get_pubkey():
    # Return a sample pubkey for testing purposes
    return "9eAsY68jFRThJdu19oB2KwaVCXC4YvsQi8kn8L9NqZWf"

@app.route("/account", methods=["POST"])
def receive_account_update():
    data = request.get_json()
    print("Received account update:")
    print(f"Pubkey: {data['pubkey']}")
    print(f"Slot: {data['slot']}")
    # Handle the account update as needed
    # ...
    return "Account update received"

if __name__ == "__main__":
    app.run(host="localhost", port=3000)
