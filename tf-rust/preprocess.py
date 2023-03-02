import tensorflow as tf

# make models/inputs directory if not exists
tf.io.gfile.makedirs("./models/inputs")

# default input shape 224x224x3
model = tf.keras.applications.MobileNetV3Small(
    input_shape=(224, 224, 3), weights="imagenet"
)

# save the model
model.save("./models/", save_format="tf")

# preprocess input images
for path in tf.io.gfile.glob("./img/*.jpg"):
    fname = path.split("/")[-1]
    img_name, ext = fname.split(".")
    buf = tf.io.read_file(path)
    img = tf.image.decode_jpeg(buf)

    # clip to the square and resize to (224, 224)
    small = tf.image.resize(img[:, 100:-100], (224, 224), antialias=True)

    # dump the content to use from Rust later
    small = tf.cast(small, tf.uint8)
    buf = tf.image.encode_png(small)
    tf.io.write_file(f"./models/inputs/{img_name}.png", buf)

    # check model prediction
    predict = model(small[tf.newaxis, :, :, :])
    predict = predict.numpy()
    decoded = tf.keras.applications.imagenet_utils.decode_predictions(predict, top=1)[0]

    print(f"""
    argmax={predict.argmax(axis=1)[0]}
    """)
    print("class_name | class_description | score")
    print("-----------+-------------------+------")
    print(f"{decoded[0][0]:>10} | {decoded[0][1]:>17} | {decoded[0][2]:0.3f}")