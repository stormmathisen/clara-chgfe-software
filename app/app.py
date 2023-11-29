import streamlit as st
import socket
from settings import Settings

if 'settings' in st.session_state:
    pass
else:
    st.session_state['settings'] = Settings()

st.title(
    "Charge Front End Control"
)
col1, col2, col3 = st.columns(3)

with col1:
    st.subheader("I/O settings")

    io_input = st.radio(
        "Input",
        ("Charge device", "Calibration", "Alternative input"),
        key = "io_input"
    )
    if io_input == "Charge device":
        st.session_state['settings'].set_io_input("EXT")
    elif io_input == "Calibration":
        st.session_state['settings'].set_io_input("CAL")
    elif io_input == "Alternative input":
        st.session_state['settings'].set_io_input("ALT")

    io_output = st.radio(
        "Output",
        ("Terminated", "Local"),
        key = "io_output"
    )
    if io_output == "Terminated":
        st.session_state['settings'].set_io_output("TERM")
    elif io_output == "Local":
        st.session_state['settings'].set_io_output("LOCAL")

    io_offset = st.radio(
        "DC Offset",
        ("Low", "High", "Manual"),
        key = "io_offset"
    )
    if io_offset == "Low":
        st.session_state['settings'].set_io_reference("REF500mV")
    elif io_offset == "High":
        st.session_state['settings'].set_io_reference("REF1000mV")
    elif io_offset == "Manual":
        st.session_state['settings'].set_io_reference("REFMANUAL")

with col2:
    st.subheader("Calibration settings")
    cal_ref_select = st.radio(
        "Calibration reference",
        ("500 mV", "1000 mV", "2048 mV", "4096 mV"),
        key = "cal_ref_select",
    )
    if cal_ref_select == "500 mV":
        st.session_state['settings'].set_calibration_reference("REF500mV")
    elif cal_ref_select == "1000 mV":
        st.session_state['settings'].set_calibration_reference("REF1000mV")
    elif cal_ref_select == "2048 mV":
        st.session_state['settings'].set_calibration_reference("REF2048mV")
    elif cal_ref_select == "4096 mV":
        st.session_state['settings'].set_calibration_reference("REF4096mV")

    level = st.number_input(
        label="Calibration level",
        min_value=0,
        max_value=255,
        value=128,
        step=1,
        key="cal_level"
    )
    st.session_state['settings'].set_calibration_level(level)

    cal_trigger = st.number_input(
        label="Calibration decimation",
        min_value=0,
        max_value=255,
        value=0,
        step=1,
        key="cal_trigger"
    )
    st.session_state['settings'].set_calibration_trigger(cal_trigger)
    st.subheader("Trigger offset")


with col3:
    st.subheader("Other settings")
    sensitivity_setting = st.radio(
        "Sensitivity setting",
        ("FB0", "FB1", "FB2", "FB3", "FB4", "FB5"),
        key = "sensitivity_setting"
    )
    st.session_state['settings'].set_integrator(sensitivity_setting)

    st.subheader("Positive rail")
    st.subheader("Negative rail")
    st.subheader("Integrator enable")

with st.sidebar:
    ip = st.text_input("IP", value="192.168.83.84")
    port = st.text_input("PORT", value = "56000")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((ip, int(port)))
    s.sendall(bytes(st.session_state['settings'].to_json()+"\n", "utf-8"))
    st.session_state.recv = s.recv(4096)

if 'recv' in st.session_state:
    st.json(st.session_state.recv.decode('utf-8'), expanded=False)
