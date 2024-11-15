pub const CSS: &str = ".info-btn {
   transition-duration: 0.18s;
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
   transform: scale(1.2);
}
.info-btn.active {
   transition-duration: 0.05s;
   transform: scale(1);
   filter: brightness(50%);
}
.next-btn {
   transition-duration: 0.165s;
   border-radius: 0px 36px 36px 0px;
   background: transparent;
   opacity: 0%;
}
.next-btn:hover {
   background-color: rgba(0, 0, 0, 0.08);
}
.next-btn.active {
   transform: scale(0.92);
}
.next-btn.focus {
   opacity: 100%;
}
.previous-btn {
   transition-duration: 0.165s;
   border-radius: 36px 0px 0px 36px;
   background: transparent;
   opacity: 0%;
}
.previous-btn:hover {
   background-color: rgba(0, 0, 0, 0.08);
}
.previous-btn.active {
   transform: scale(0.92);
}
.previous-btn.focus {
   opacity: 100%;
}
.web-content {
   border-radius: 36px;
   background: transparent;
}
";