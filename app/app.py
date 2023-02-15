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
    if cal_ref_select == "500 mV":
        settings.set_calibration_reference("REF500mV")
    elif cal_ref_select == "1000 mV":
        settings.set_calibration_reference("REF1000mV")
    elif cal_ref_select == "2048 mV":
        settings.set_calibration_reference("REF2048mV")
    elif cal_ref_select == "4096 mV":
        settings.set_calibration_reference("REF4096mV")

    level = st.number_input(
        label="Calibration level",
        min_value=0,
        max_value=255,
        value=128,
        step=1,
        key="cal_level"
    )
    settings.set_calibration_level(level)

    st.subheader("Trigger decimation")
    st.subheader("Trigger offset")


with col3:
    sensitivity_setting = st.radio(
        "Sensitivity setting",
        ("FB0", "FB1", "FB2", "FB3", "FB4", "FB5"),
        key = "sensitivity_setting"
    )
    settings.set_integrator(sensitivity_setting)

    st.header("Other settings")
    st.subheader("Positive rail")
    st.subheader("Negative rail")
    st.subheader("Integrator enable")

with st.sidebar:
    ip = st.text_input("IP", value="192.168.83.84")
    port = st.text_input("PORT", value = "56000")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((ip, int(port)))
    s.sendall(bytes(settings.to_json()+"\n", "utf-8"))
    st.session_state.recv = s.recv(4096)

if 'recv' in st.session_state:
    st.json(st.session_state.recv.decode('utf-8'), expanded=False)
