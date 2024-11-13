pub const CSS: &str = ".info-btn {
   transition-duration: 0.24s;
   filter: brightness(100%);
	color: white;
	background: transparent;
   min-width: 28px;
   min-height: 28px;
   transform: scale(1);
   margin-top: 10px;
   margin-bottom: 10px;
}
.info-btn:hover {
   transform: scale(1.25);
}
.info-btn-clicked {
   transform: scale(1) !important;
   filter: brightness(50%) !important;
}
";