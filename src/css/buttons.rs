pub const CSS: &str = ".info-btn {
   transition-duration: 0.24s;
   filter: brightness(100%);
	color: white;
	background: transparent;
   min-width: 31px;
   min-height: 31px;
   transform: scale(1);
   margin-top: 5px;
   margin-bottom: 5px;
}
.info-btn:hover {
   transform: scale(1.25);
}
.info-btn.active {
   transform: scale(1);
   filter: brightness(50%);
}
";