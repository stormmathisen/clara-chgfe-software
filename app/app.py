import streamlit as st
import socket
from settings import Settings

settings = Settings()

st.title(
    "Charge Front End Control"
)
col1, col2, col3 = st.columns(3)

with col1:
    st.header("I/O settings")

    io_input = st.radio(
        "Input",
        ("Charge device", "Calibration", "Alternative input"),
        key = "io_input"
    )
    if io_input == "Charge device":
        settings.set_io_input("EXT")
    elif io_input == "Calibration":
        settings.set_io_input("CAL")
    elif io_input == "Alternative input":
        settings.set_io_input("ALT")

    io_output = st.radio(
        "Output",
        ("Terminated", "Local"),
        key = "io_output"
    )
    if io_output == "Terminated":
        settings.set_io_output("TERM")
    elif io_output == "Local":
        settings.set_io_output("LOCAL")

    io_offset = st.radio(
        "DC Offset",
        ("Low", "High", "Manual"),
        key = "io_offset"
    )
    if io_offset == "Low":
        settings.set_io_reference("REF500mV")
    elif io_offset == "High":
        settings.set_io_reference("REF1000mV")
    elif io_offset == "Manual":
        settings.set_io_reference("REFMANUAL")

with col2:
    st.header("Calibration settings")
    cal_ref_select = st.radio(
        "Calibration reference",
        ("500 mV", "1000 mV", "2048 mV", "4096 mV"),
        key = "cal_ref_select",
    )
    st.subheader("Calibration level")
    level = st.text_input(
        'Calibration level',
        value='128',
        key='cal_level')
    if int(level) > 255:
        st.warning("Level has to be between 0 and 255")
        level = '255'
    elif int(level) < 0:
        st.warning("Level has to be between 0 and 255")
        level = '0'

    st.subheader("Trigger decimation")
    st.subheader("Trigger offset")


with col3:
    sensitivity_setting = st.radio(
        "Sensitivity setting",
        ("FB0", "FB1", "FB2", "FB3", "FB4", "FB5"),
        key = "sensitivity_setting"
    )
    st.header("Other settings")
    st.subheader("Positive rail")
    st.subheader("Negative rail")
    st.subheader("Integrator enable")

with st.sidebar:
    ip = st.text_input("IP", value="192.168.83.84")
    port = st.text_input("PORT", value = "56000")

def sendMessage(ip, port, message):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((ip, int(port)))
        s.sendall(bytes(message, "utf-8"))
        return s.recv(4096)

if cal_ref_select == "500 mV":
    st.session_state['recv'] = sendMessage(ip, port, "field=calibration.reference&setting=500mv\n")
elif cal_ref_select == "1000 mV":
    st.session_state['recv'] = sendMessage(ip, port, "field=calibration.reference&setting=1000mv\n")
elif cal_ref_select == "2048 mV":
    st.session_state['recv'] = sendMessage(ip, port, "field=calibration.reference&setting=2048mv\n")
elif cal_ref_select == "4096 mV":
    st.session_state['recv'] = sendMessage(ip, port, "field=calibration.reference&setting=4096mv\n")

st.session_state['recv'] = sendMessage(ip, port, "field=calibration.level&setting="+level+"\n")

if sensitivity_setting == "FB0":
    st.session_state['recv'] = sendMessage(ip, port, "field=integrator&setting=fb0\n")
elif sensitivity_setting == "FB1":
    st.session_state['recv'] = sendMessage(ip, port, "field=integrator&setting=fb1\n")
elif sensitivity_setting == "FB2":
    st.session_state['recv'] = sendMessage(ip, port, "field=integrator&setting=fb2\n")
elif sensitivity_setting == "FB3":
    st.session_state['recv'] = sendMessage(ip, port, "field=integrator&setting=fb3\n")
elif sensitivity_setting == "FB4":
    st.session_state['recv'] = sendMessage(ip, port, "field=integrator&setting=fb4\n")
elif sensitivity_setting == "FB5":
    st.session_state['recv'] = sendMessage(ip, port, "field=integrator&setting=fb5\n")

if io_input == "Charge device":
    st.session_state['recv'] = sendMessage(ip, port, "field=io.input&setting=ext\n")
elif io_input == "Calibration":
    st.session_state['recv'] = sendMessage(ip, port, "field=io.input&setting=cal\n")
elif io_input == "Alternative input":
    st.session_state['recv'] = sendMessage(ip, port, "field=io.input&setting=alt\n")


if 'recv' in st.session_state:
    st.json(st.session_state.recv.decode('utf-8'), expanded=False)
