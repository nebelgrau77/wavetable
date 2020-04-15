pub mod wavetable {

    ///returns sine and cosine values for angles expressed in degrees (only discrete values)

    pub fn sin_getter(angle: i16) -> f32 {
    
        let idx = angle%360;
        let qtr = (angle/90)%4;
        
        let sin = match qtr {
            0 => VALUES[idx as usize],
            1 => VALUES[(180-idx) as usize],
            2 => -VALUES[(idx-180) as usize],
            3 => -VALUES[(360-idx) as usize],
            _ => 0.0,
        };
        
        return sin;
    }

    pub fn cos_getter(angle: i16) -> f32 {
        let new_angle = angle + 90;
        let cos = sin_getter(new_angle);
        
        return cos;
        
    }


    static VALUES: [f32; 91] =
    [0.0,
    0.01745240643728351,
    0.03489949670250097,
    0.052335956242943835,
    0.0697564737441253,
    0.08715574274765817,
    0.10452846326765347,
    0.12186934340514748,
    0.13917310096006544,
    0.15643446504023087,
    0.17364817766693033,
    0.1908089953765448,
    0.20791169081775934,
    0.224951054343865,
    0.24192189559966773,
    0.25881904510252074,
    0.27563735581699916,
    0.29237170472273677,
    0.3090169943749474,
    0.3255681544571567,
    0.3420201433256687,
    0.35836794954530027,
    0.374606593415912,
    0.39073112848927377,
    0.4067366430758002,
    0.42261826174069944,
    0.4383711467890774,
    0.45399049973954675,
    0.4694715627858908,
    0.48480962024633706,
    0.49999999999999994,
    0.5150380749100542,
    0.5299192642332049,
    0.5446390350150271,
    0.5591929034707469,
    0.573576436351046,
    0.5877852522924731,
    0.6018150231520483,
    0.6156614753256583,
    0.6293203910498374,
    0.6427876096865393,
    0.6560590289905073,
    0.6691306063588582,
    0.6819983600624985,
    0.6946583704589973,
    0.7071067811865475,
    0.7193398003386511,
    0.7313537016191705,
    0.7431448254773942,
    0.754709580222772,
    0.766044443118978,
    0.7771459614569709,
    0.788010753606722,
    0.7986355100472928,
    0.8090169943749475,
    0.8191520442889918,
    0.8290375725550417,
    0.838670567945424,
    0.848048096156426,
    0.8571673007021123,
    0.8660254037844386,
    0.8746197071393957,
    0.8829475928589269,
    0.8910065241883678,
    0.898794046299167,
    0.9063077870366499,
    0.9135454576426009,
    0.9205048534524404,
    0.9271838545667874,
    0.9335804264972017,
    0.9396926207859083,
    0.9455185755993167,
    0.9510565162951535,
    0.9563047559630354,
    0.9612616959383189,
    0.9659258262890683,
    0.9702957262759965,
    0.9743700647852352,
    0.9781476007338056,
    0.981627183447664,
    0.984807753012208,
    0.9876883405951378,
    0.9902680687415704,
    0.992546151641322,
    0.9945218953682733,
    0.9961946980917455,
    0.9975640502598242,
    0.9986295347545738,
    0.9993908270190958,
    0.9998476951563913,
    1.0];

}

#[cfg(test)]
mod tests {
    
    use super::wavetable::*;
    
    #[test]
    fn sin_halfpi() {
        
        let val = sin_getter(90);

        assert_eq!(val, 1.0);
    }

    #[test]
    fn sin_pi() {
        
        let val = sin_getter(180);

        assert_eq!(val, 0.0);
    }

    #[test]
    fn sin_piandahalf() {
        
        let val = sin_getter(270);

        assert_eq!(val, -1.0);
    }


    #[test]
    fn sin_twopi() {
        
        let val = sin_getter(360);

        assert_eq!(val, 0.0);
    }


    #[test]
    fn cos_zero() {
        
        let val = cos_getter(0);

        assert_eq!(val, 1.0);
    }

    #[test]
    fn cos_halfpi() {
        
        let val = cos_getter(90);

        assert_eq!(val, 0.0);
    }

    #[test]
    fn cos_pi() {
        
        let val = cos_getter(180);

        assert_eq!(val, -1.0);
    }

}
