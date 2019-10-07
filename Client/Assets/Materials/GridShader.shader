// Upgrade NOTE: upgraded instancing buffer 'Props' to new syntax.

Shader "PDT Shaders/TestGrid"
{
	Properties
    {
		_LineColor ("Line Color", Color) = (1,1,1,1)
		_CellColor ("Cell Color", Color) = (0,0,0,0)
		[PerRendererData] _MainTex ("Albedo (RGB)", 2D) = "white" {}
		[IntRange] _GridSize("Grid Size", Range(1,100)) = 10
		_LineSize("Line Size", Range(0,1)) = 0.15
		_Radius("Edge Radius", Range(0,1)) = 0.15
        _XMult("X Edge Multiplier", Range(1, 16)) = 4
        _YMult("Y Edge Multiplier", Range(1, 16)) = 4
	}
	SubShader
    {
		Tags { "Queue"="AlphaTest" "RenderType"="TransparentCutout" }
		LOD 200
	

		CGPROGRAM
		// Physically based Standard lighting model, and enable shadows on all light types
		#pragma surface surf Standard fullforwardshadows

		// Use shader model 3.0 target, to get nicer looking lighting
		#pragma target 3.0

		sampler2D _MainTex;

		struct Input
        {
			float2 uv_MainTex;
		};

		float4 _LineColor;
		float4 _CellColor;

		float _GridSize;
		float _LineSize;
		float _Radius;
        float _XMult;
        float _YMult;

		void surf (Input IN, inout SurfaceOutputStandard o)
        {
			float2 uv = IN.uv_MainTex;

			fixed4 c = float4(0.0,0.0,0.0,0.0);
			float brightness = 1.;
			float gsize = floor(_GridSize);
            float half_ls = _LineSize / 2.;

			float2 id;

			id.x = floor(uv.x/(1.0/gsize));
			id.y = floor(uv.y/(1.0/gsize));

			float4 color = _CellColor;
			brightness = _CellColor.w;

            float vx = frac(uv.x*gsize);
            float vy = frac(uv.y*gsize);
            float vx_2 = 1 - frac(uv.x*gsize);
            float vy_2 = 1 - frac(uv.y*gsize);
            
            // TODO: Clean this up a bit
			if (vx <= half_ls || vy <= half_ls || vx_2 <= half_ls || vy_2 <= half_ls || 
               (_XMult*(vx-0.5)*(vx-0.5) + (vy-0.5)*(vy-0.5) >  _Radius) ||
               ((vx-0.5)*(vx-0.5) + _YMult*(vy-0.5)*(vy-0.5) >  _Radius))
			{
				brightness = _LineColor.w;
				color = _LineColor;
			}

			//Clip transparent spots using alpha cutout
			if (brightness == 0.0)
            {
				clip(c.a - 1.0);
			}
			

			o.Albedo = color * brightness;
		}
		ENDCG
	}
	FallBack "Diffuse"
}
