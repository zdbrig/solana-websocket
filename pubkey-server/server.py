from flask import Flask, request

app = Flask(__name__)

@app.route("/pubkey", methods=["GET"])
def get_pubkey():
    # Return a sample pubkey for testing purposes
    return "CxXDXjGBJ6RwMKKLqkd9KCAR5yfswNd8iXcQPmFFeDvU,CYnzvNjvNJrgN8L9Q9wT6PPd19zy1qfmFAjjrL7RprrP"

@app.route("/account", methods=["POST"])
def receive_account_update():
    data = request.get_json()
    print("Received account update:")
    print(f"Pubkey: {data['pubkey']}")
    print(f"Data: {data['data']}")
    # Handle the account update as needed
    # ...
    return "Account update received"

if __name__ == "__main__":
    app.run(host="localhost", port=3000)
