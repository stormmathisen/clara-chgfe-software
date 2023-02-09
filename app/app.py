import streamlit as st

st.write(
    "HELLO ***PERSON***"
)
rktvs = st.radio("Label", 
("Go away",
"No really",
"I fucking mean is")
)

if rktvs == "Go away":
    st.write("I said go away")
elif rktvs == "No really":
    st.write("I really fucking mean it")